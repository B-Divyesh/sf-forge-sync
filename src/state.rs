use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde::Serialize;
use std::{fs, path::Path};

pub struct State {
    conn: Connection,
    writable: bool,
}

#[derive(Debug, Serialize)]
pub struct Status {
    pub repositories: i64,
    #[serde(rename = "record_links")]
    pub mappings: i64,
    #[serde(rename = "run_history_entries")]
    pub audit_events: i64,
    pub last_success_at: Option<String>,
}

impl State {
    pub fn open(dir: &Path) -> Result<Self> {
        fs::create_dir_all(dir)
            .with_context(|| format!("create state directory {}", dir.display()))?;
        let conn = Connection::open(dir.join("state.sqlite3")).context("open state database")?;
        initialize(&conn)?;
        Ok(Self {
            conn,
            writable: true,
        })
    }

    /// Opens existing synchronization state without creating or modifying any files.
    /// A first dry run uses an in-memory empty state, so it has no durable footprint.
    pub fn open_read_only(dir: &Path) -> Result<Self> {
        let path = dir.join("state.sqlite3");
        let conn = if path.is_file() {
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
                .context("open state database read-only")?
        } else {
            let conn = Connection::open_in_memory().context("open temporary dry-run state")?;
            initialize(&conn)?;
            conn
        };
        Ok(Self {
            conn,
            writable: false,
        })
    }
    pub fn mapping(&self, kind: &str, repo: &str, source_id: i64) -> Result<Option<(i64, String)>> {
        self.conn.query_row("SELECT target_id,source_updated FROM mappings WHERE source_kind=?1 AND repo=?2 AND source_id=?3", params![kind,repo,source_id], |r| Ok((r.get(0)?,r.get(1)?))).optional().map_err(Into::into)
    }
    pub fn map(
        &self,
        kind: &str,
        repo: &str,
        source_id: i64,
        target_id: i64,
        updated: &str,
    ) -> Result<()> {
        if !self.writable {
            return Ok(());
        }
        self.conn.execute("INSERT INTO mappings(source_kind,repo,source_id,target_id,source_updated) VALUES(?1,?2,?3,?4,?5) ON CONFLICT(source_kind,repo,source_id) DO UPDATE SET target_id=excluded.target_id,source_updated=excluded.source_updated", params![kind,repo,source_id,target_id,updated])?;
        Ok(())
    }
    pub fn repository_ok(&self, name: &str, source_id: i64, target_id: &str) -> Result<()> {
        if !self.writable {
            return Ok(());
        }
        self.conn.execute("INSERT INTO repositories(name,source_id,target_id,last_success_at,last_error) VALUES(?1,?2,?3,?4,NULL) ON CONFLICT(name) DO UPDATE SET source_id=excluded.source_id,target_id=excluded.target_id,last_success_at=excluded.last_success_at,last_error=NULL", params![name,source_id,target_id,Utc::now().to_rfc3339()])?;
        Ok(())
    }
    pub fn repository_error(&self, name: &str, source_id: i64, detail: &str) -> Result<()> {
        if !self.writable {
            return Ok(());
        }
        self.conn.execute("INSERT INTO repositories(name,source_id,target_id,last_error) VALUES(?1,?2,'',?3) ON CONFLICT(name) DO UPDATE SET last_error=excluded.last_error", params![name,source_id,detail])?;
        Ok(())
    }
    pub fn audit(
        &self,
        action: &str,
        repo: Option<&str>,
        kind: Option<&str>,
        source_id: Option<i64>,
        detail: &str,
    ) -> Result<()> {
        if !self.writable {
            return Ok(());
        }
        self.conn.execute("INSERT INTO audit(at,action,repo,object_kind,source_id,detail) VALUES(?1,?2,?3,?4,?5,?6)", params![Utc::now().to_rfc3339(),action,repo,kind,source_id,detail])?;
        Ok(())
    }
    pub fn cache_get(&self, key: &str) -> Result<Option<String>> {
        self.conn
            .query_row("SELECT value FROM cache WHERE key=?1", [key], |r| r.get(0))
            .optional()
            .map_err(Into::into)
    }
    pub fn cache_set(&self, key: &str, value: &str) -> Result<()> {
        if !self.writable {
            return Ok(());
        }
        self.conn.execute("INSERT INTO cache(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value", params![key,value])?;
        Ok(())
    }
    pub fn status(&self) -> Result<Status> {
        Ok(Status {
            repositories: self
                .conn
                .query_row("SELECT COUNT(*) FROM repositories", [], |r| r.get(0))?,
            mappings: self
                .conn
                .query_row("SELECT COUNT(*) FROM mappings", [], |r| r.get(0))?,
            audit_events: self
                .conn
                .query_row("SELECT COUNT(*) FROM audit", [], |r| r.get(0))?,
            last_success_at: self.conn.query_row(
                "SELECT MAX(last_success_at) FROM repositories",
                [],
                |r| r.get(0),
            )?,
        })
    }
}

fn initialize(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;
        CREATE TABLE IF NOT EXISTS mappings(source_kind TEXT NOT NULL, repo TEXT NOT NULL, source_id INTEGER NOT NULL, target_id INTEGER NOT NULL, source_updated TEXT NOT NULL DEFAULT '', PRIMARY KEY(source_kind,repo,source_id));
        CREATE TABLE IF NOT EXISTS repositories(name TEXT PRIMARY KEY, source_id INTEGER NOT NULL, target_id TEXT NOT NULL, last_success_at TEXT, last_error TEXT);
        CREATE TABLE IF NOT EXISTS audit(id INTEGER PRIMARY KEY, at TEXT NOT NULL, action TEXT NOT NULL, repo TEXT, object_kind TEXT, source_id INTEGER, detail TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS cache(key TEXT PRIMARY KEY, value TEXT NOT NULL);",
    )?;
    Ok(())
}
