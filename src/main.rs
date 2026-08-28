use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use forge_sync::{
    archive::Archive,
    engine,
    forge::Forge,
    github::GitHub,
    model::{Comment, Issue, ItemSnapshot, Label, PullRef, Repository, User},
    state::State,
    Config,
};
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
    long_about = "Discovers repositories in a GitHub organization, mirrors Git branches, tags, and issue or pull-request metadata to Forgejo, Codeberg, or GitLab, and writes a JSON archive."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate configuration, credentials, and both API endpoints without writing
    Doctor(Common),
    /// Perform one synchronization pass
    Sync(SyncArgs),
    /// Poll and synchronize continuously until SIGINT or SIGTERM
    Daemon(Common),
    /// Show local synchronization state
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
                    "Repositories: {}\nLinks between GitHub and target records: {}\nDated run history entries: {}\nLast success: {}",
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
    let repository_fixture: serde_json::Value =
        serde_json::from_str(include_str!("../examples/sample-mirror/repository.json"))?;
    let pull_fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../examples/sample-mirror/pull-request-41.json"
    ))?;
    let repository = Repository {
        id: 1,
        name: repository_fixture["repository"]
            .as_str()
            .context("demo repository name")?
            .into(),
        full_name: format!(
            "{}/{}",
            repository_fixture["organization"]
                .as_str()
                .context("demo organization")?,
            repository_fixture["repository"]
                .as_str()
                .context("demo repository name")?
        ),
        description: Some("Tide alerts and harbor notices".into()),
        clone_url: "https://github.example/harbor-coop/harbor-tools.git".into(),
        html_url: "https://github.example/harbor-coop/harbor-tools".into(),
        archived: false,
        private: true,
        default_branch: "main".into(),
        updated_at: "2026-08-28T08:00:00Z".into(),
    };
    let author = User {
        login: pull_fixture["author"]
            .as_str()
            .context("demo author")?
            .into(),
        html_url: "https://github.example/marina".into(),
    };
    let issue = Issue {
        id: 4100,
        number: pull_fixture["number"]
            .as_i64()
            .context("demo pull-request number")?,
        title: pull_fixture["title"]
            .as_str()
            .context("demo pull-request title")?
            .into(),
        body: Some("Make warning thresholds and tide times readable on a phone.".into()),
        state: "open".into(),
        user: author.clone(),
        labels: vec![],
        milestone: None,
        pull_request: Some(PullRef {
            url: pull_fixture["source_url"]
                .as_str()
                .context("demo source URL")?
                .into(),
        }),
        html_url: pull_fixture["source_url"]
            .as_str()
            .context("demo source URL")?
            .into(),
        created_at: "2026-08-27T09:00:00Z".into(),
        updated_at: "2026-08-28T08:00:00Z".into(),
    };
    let comments = vec![
        Comment {
            id: 4101,
            body: pull_fixture["reviews"][0]["body"]
                .as_str()
                .context("demo review")?
                .into(),
            user: User {
                login: "keon".into(),
                html_url: "https://github.example/keon".into(),
            },
            html_url: format!("{}#pullrequestreview-4101", issue.html_url),
            created_at: "2026-08-27T10:00:00Z".into(),
            updated_at: "2026-08-27T10:00:00Z".into(),
            kind: "review (approved)".into(),
            path: None,
            line: None,
            issue_url: None,
        },
        Comment {
            id: 4102,
            body: pull_fixture["inline_comments"][0]["body"]
                .as_str()
                .context("demo inline comment")?
                .into(),
            user: User {
                login: "keon".into(),
                html_url: "https://github.example/keon".into(),
            },
            html_url: format!("{}#discussion_r4102", issue.html_url),
            created_at: "2026-08-27T10:05:00Z".into(),
            updated_at: "2026-08-27T10:05:00Z".into(),
            kind: "inline review comment".into(),
            path: Some(
                pull_fixture["inline_comments"][0]["path"]
                    .as_str()
                    .context("demo comment path")?
                    .into(),
            ),
            line: pull_fixture["inline_comments"][0]["line"].as_i64(),
            issue_url: None,
        },
        Comment {
            id: 4103,
            body: pull_fixture["discussion_comments"][0]["body"]
                .as_str()
                .context("demo discussion comment")?
                .into(),
            user: author,
            html_url: format!("{}#issuecomment-4103", issue.html_url),
            created_at: "2026-08-27T10:10:00Z".into(),
            updated_at: "2026-08-27T10:10:00Z".into(),
            kind: "comment".into(),
            path: None,
            line: None,
            issue_url: None,
        },
    ];
    let pull_label = Label {
        id: -1,
        name: "forge-sync:pull-request".into(),
        color: "174d67".into(),
        description: Some("Mirrored GitHub pull request discussion".into()),
    };
    fs::write(
        root.join("README.md"),
        "# forge-sync completed sample mirror\n\nThis disposable directory was built from the shipped Harbor Cooperative records using forge-sync's archive, record-link, run-history, and rendering code.\n",
    )?;
    fs::write(
        root.join("target/harbor-tools/branches.txt"),
        "main\nrelease/2026\n",
    )?;
    fs::write(root.join("target/harbor-tools/tags.txt"), "v2.4.0\n")?;
    fs::write(
        root.join("target/harbor-tools/issues/41.md"),
        format!(
            "# {}\n\n{}\n\n{}",
            issue.mirrored_title(),
            issue.mirrored_body(),
            comments
                .iter()
                .map(Comment::mirrored_body)
                .collect::<Vec<_>>()
                .join("\n\n")
        ),
    )?;
    let state = State::open(&root.join("state"))?;
    state.map("item", &repository.name, issue.id, 41, &issue.updated_at)?;
    state.audit(
        "start",
        Some(&repository.name),
        Some("repository"),
        Some(repository.id),
        "sample synchronization started",
    )?;
    state.audit(
        "copied",
        Some(&repository.name),
        Some("pull request"),
        Some(issue.id),
        "sample pull-request record copied",
    )?;
    state.repository_ok(&repository.name, repository.id, "41")?;
    state.audit(
        "complete",
        Some(&repository.name),
        Some("repository"),
        Some(repository.id),
        "sample archive committed",
    )?;
    let archive = Archive::new(&root.join("archive"), true)?;
    archive.write_repository(
        &repository,
        &[pull_label],
        &[],
        &[ItemSnapshot { issue, comments }],
    )?;
    archive.commit()?;

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
        println!("  links between GitHub and target records: 1 · dated run history entries: 3");
        println!("  JSON archive: committed");
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
