use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use forge_sync::{engine, forge::Forge, github::GitHub, state::State, Config};
use serde_json::json;
use std::{
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
    if e.contains("config") || e.contains("environment variable") || e.contains("toml") {
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
