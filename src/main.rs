use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use forge_sync::{engine, forge::Forge, github::GitHub, state::State, Config};
use serde_json::json;
use std::{
    fs,
    path::PathBuf,
    process::ExitCode,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

#[derive(Parser)]
#[command(
    name = "forge-sync",
    version,
    about = "Continuously mirror a GitHub organization to an independent forge",
    long_about = "Discovers every repository in a GitHub organization, mirrors Git refs and issue/PR metadata to Forgejo, Codeberg, or GitLab, and writes a portable JSON archive. Never prompts or sends telemetry."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate configuration, credentials, and both API endpoints without writing
    Doctor(Common),
    /// Perform one idempotent synchronization pass
    Sync(SyncArgs),
    /// Poll and synchronize continuously until SIGINT or SIGTERM
    Daemon(Common),
    /// Read local synchronization state without contacting either forge
    Status(Common),
    /// Create and show a completed, isolated sample mirror without tokens
    Demo {
        /// Emit the sample location and summary as JSON
        #[arg(long)]
        json: bool,
    },
    /// Print configuration resources
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}
#[derive(Subcommand)]
enum ConfigCommand {
    /// Print a complete annotated TOML configuration
    Example,
}
#[derive(Args)]
struct Common {
    /// Path to forge-sync.toml
    #[arg(short, long, default_value = "forge-sync.toml")]
    config: PathBuf,
    /// Emit one JSON document to stdout
    #[arg(long)]
    json: bool,
}
#[derive(Args)]
struct SyncArgs {
    #[command(flatten)]
    common: Common,
    /// Plan API writes and git mirroring without changing the target
    #[arg(long)]
    dry_run: bool,
}

fn main() -> ExitCode {
    match execute(Cli::parse()) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("forge-sync: {error:#}");
            ExitCode::from(classify(&format!("{error:#}")))
        }
    }
}
fn execute(cli: Cli) -> Result<u8> {
    match cli.command {
        Command::Config {
            command: ConfigCommand::Example,
        } => {
            print!("{}", forge_sync::config::EXAMPLE);
            Ok(0)
        }
        Command::Doctor(args) => {
            let c = Config::from_path(&args.config)?;
            let source = GitHub::new(&c.source.api_url, &c.source.org, c.source_token()?)?;
            let target = Forge::new(c.target.clone(), c.target_token()?)?;
            let gh = source.doctor().context("authenticate with GitHub")?;
            let forge = target.doctor().context("authenticate with target forge")?;
            if args.json {
                println!(
                    "{}",
                    serde_json::to_string(&json!({"ok":true,"github":gh.login,"target":forge}))?
                );
            } else {
                println!("✓ GitHub authenticated as {}\n✓ Target authenticated as {}\n✓ Configuration is ready", gh.login, forge);
            }
            Ok(0)
        }
        Command::Status(args) => {
            let c = Config::from_path(&args.config)?;
            let status = State::open(&c.sync.state_dir)?.status()?;
            if args.json {
                println!("{}", serde_json::to_string(&status)?);
            } else {
                println!(
                    "Repositories: {}\nMappings: {}\nAudit events: {}\nLast success: {}",
                    status.repositories,
                    status.mappings,
                    status.audit_events,
                    status.last_success_at.as_deref().unwrap_or("never")
                );
            }
            Ok(0)
        }
        Command::Demo { json } => run_demo(json),
        Command::Sync(args) => {
            let mut c = Config::from_path(&args.common.config)?;
            c.sync.dry_run |= args.dry_run;
            let report = engine::run_once(&c)?;
            print_report(&report, args.common.json)?;
            Ok(if report.failed > 0 { 6 } else { 0 })
        }
        Command::Daemon(args) => {
            let c = Config::from_path(&args.config)?;
            c.source_token()?;
            c.target_token()?;
            let running = Arc::new(AtomicBool::new(true));
            let flag = running.clone();
            ctrlc::set_handler(move || flag.store(false, Ordering::SeqCst))?;
            while running.load(Ordering::SeqCst) {
                match engine::run_once(&c) {
                    Ok(report) => print_report(&report, args.json)?,
                    Err(error) if args.json => println!(
                        "{}",
                        serde_json::to_string(&json!({"ok":false,"error":error.to_string()}))?
                    ),
                    Err(error) => eprintln!("forge-sync: sync pass failed; retrying: {error:#}"),
                }
                for _ in 0..c.sync.interval_seconds {
                    if !running.load(Ordering::SeqCst) {
                        break;
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
            Ok(0)
        }
    }
}

/// Make a deterministic, inspectable sample without reading configuration,
/// environment tokens, or the current directory.  This is intentionally not a
/// shortcut into a user's state directory: the demo is always a new directory
/// below the operating system temporary directory.
fn run_demo(json_output: bool) -> Result<u8> {
    let nonce = format!(
        "{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );
    let root = std::env::temp_dir().join(format!("forge-sync-demo-{nonce}"));
    fs::create_dir_all(root.join("target/harbor-tools/issues"))?;
    fs::create_dir_all(root.join("archive/repositories/harbor-tools/items"))?;
    fs::create_dir_all(root.join("state"))?;
    fs::write(
        root.join("README.md"),
        "# forge-sync completed sample mirror\n\nThis directory is disposable demo output. It contains a target-style record, local source-to-target mappings, audit events, and a JSON archive for the fictional Harbor Cooperative organization.\n",
    )?;
    fs::write(
        root.join("target/harbor-tools/branches.txt"),
        "main\nrelease/2026\n",
    )?;
    fs::write(root.join("target/harbor-tools/tags.txt"), "v2.4.0\n")?;
    fs::write(
        root.join("target/harbor-tools/issues/41.md"),
        "# [pull request] Make tide alerts readable\n\nAuthor: marina\nSource: https://github.example/harbor-coop/harbor-tools/pull/41\n\nReview: Please keep the alert threshold visible.\n\nInline comment: src/alerts.rs:18 — use the harbor timezone.\n",
    )?;
    fs::write(
        root.join("archive/repositories/harbor-tools/items/41.json"),
        include_str!("../examples/sample-mirror/pull-request-41.json"),
    )?;
    fs::write(
        root.join("archive/repositories/harbor-tools/repository.json"),
        include_str!("../examples/sample-mirror/repository.json"),
    )?;
    fs::write(
        root.join("state/id-mappings.json"),
        "{\n  \"github:issue:harbor-tools:41\": \"target:issue:harbor-tools:41\"\n}\n",
    )?;
    fs::write(
        root.join("state/audit-events.jsonl"),
        "{\"event\":\"repository.created\",\"repository\":\"harbor-tools\"}\n{\"event\":\"pull_request.copied\",\"number\":41}\n{\"event\":\"archive.written\",\"path\":\"repositories/harbor-tools/items/41.json\"}\n",
    )?;

    let git = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(root.join("archive"))
        .status()
        .context("initialize the demo JSON archive")?;
    if !git.success() {
        anyhow::bail!("initialize the demo JSON archive failed");
    }
    for (key, value) in [
        ("user.name", "forge-sync demo"),
        ("user.email", "demo@forge-sync.invalid"),
    ] {
        let status = std::process::Command::new("git")
            .args(["config", key, value])
            .current_dir(root.join("archive"))
            .status()?;
        if !status.success() {
            anyhow::bail!("configure the demo JSON archive failed");
        }
    }
    let status = std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(root.join("archive"))
        .status()?;
    if !status.success() {
        anyhow::bail!("stage the demo JSON archive failed");
    }
    let status = std::process::Command::new("git")
        .args(["commit", "-qm", "archive completed sample mirror"])
        .current_dir(root.join("archive"))
        .status()?;
    if !status.success() {
        anyhow::bail!("commit the demo JSON archive failed");
    }

    if json_output {
        println!(
            "{}",
            serde_json::to_string(
                &json!({"ok":true,"path":root,"repositories":1,"issues":1,"pull_request_records":1,"archive_commit":true})
            )?
        );
    } else {
        println!("Completed sample mirror — no tokens used");
        println!("  repository: harbor-tools");
        println!("  branches: 2 · tags: 1 · issue records: 1 · pull-request records: 1");
        println!("  source-to-target mappings: 1 · audit events: 3 · JSON archive: committed");
        println!("Demo output: {}", root.display());
        println!("Remove that directory when you are done; forge-sync did not read or write your configuration.");
    }
    Ok(0)
}
fn print_report(report: &engine::RunReport, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string(report)?);
    } else {
        println!(
            "Discovered {} · synchronized {} · failed {} · issues {} · comments {} · {} ms{}",
            report.discovered,
            report.synchronized,
            report.failed,
            report.issues,
            report.comments,
            report.duration_ms,
            if report.dry_run { " · dry run" } else { "" }
        );
        for e in &report.errors {
            eprintln!("  {}: {}", e.repository, e.error);
        }
    }
    Ok(())
}
fn classify(error: &str) -> u8 {
    let e = error.to_lowercase();
    if e.contains("configuration error") {
        2
    } else if e.contains("401") || e.contains("403") || e.contains("authenticate") {
        3
    } else if e.contains("api") || e.contains("rate") {
        4
    } else if e.contains("git") {
        5
    } else {
        1
    }
}
