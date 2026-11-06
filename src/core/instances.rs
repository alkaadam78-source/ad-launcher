use crate::utils::fs::{default_minecraft_dir, ensure_dir, launcher_config_dir};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub mc_version: String,
    pub loader: String,
    pub loader_version: String,
    pub ram_mb: u32,
    pub game_dir: PathBuf,
    pub java_path: Option<PathBuf>,
    pub created_at: String,
    pub last_played_at: Option<String>,
}

pub struct InstanceManager {
    conn: Connection,
}

impl InstanceManager {
    pub fn new() -> rusqlite::Result<Self> {
        let db_dir = launcher_config_dir();
        let _ = ensure_dir(&db_dir);
        let db_path = db_dir.join("launcher.db");

        let conn = Connection::open(db_path)?;
        let manager = Self { conn };
        manager.init_db()?;
        manager.ensure_default_instance()?;
        Ok(manager)
    }

    fn init_db(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS instances (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                mc_version TEXT NOT NULL,
                loader TEXT NOT NULL,
                loader_version TEXT NOT NULL,
                ram_mb INTEGER NOT NULL,
                game_dir TEXT NOT NULL,
                java_path TEXT,
                created_at TEXT NOT NULL,
                last_played_at TEXT
            )",
            [],
        )?;
        Ok(())
    }

    fn ensure_default_instance(&self) -> rusqlite::Result<()> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM instances", [], |row| row.get(0))?;

        if count == 0 {
            let default_dir = default_minecraft_dir();
            let now = chrono::Utc::now().to_rfc3339();
            let default_inst = Instance {
                id: "default-1.20.1-fabric".to_string(),
                name: "Minecraft 1.20.1 (Fabric Optimized)".to_string(),
                mc_version: "1.20.1".to_string(),
                loader: "fabric".to_string(),
                loader_version: "0.15.11".to_string(),
                ram_mb: 3072,
                game_dir: default_dir,
                java_path: None,
                created_at: now,
                last_played_at: None,
            };
            self.create_instance(&default_inst)?;
        }
        Ok(())
    }

    pub fn list_instances(&self) -> rusqlite::Result<Vec<Instance>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, mc_version, loader, loader_version, ram_mb, game_dir, java_path, created_at, last_played_at
             FROM instances ORDER BY last_played_at DESC NULLS LAST, name ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let game_dir_str: String = row.get(6)?;
            let java_path_str: Option<String> = row.get(7)?;

            Ok(Instance {
                id: row.get(0)?,
                name: row.get(1)?,
                mc_version: row.get(2)?,
                loader: row.get(3)?,
                loader_version: row.get(4)?,
                ram_mb: row.get(5)?,
                game_dir: PathBuf::from(game_dir_str),
                java_path: java_path_str.map(PathBuf::from),
                created_at: row.get(8)?,
                last_played_at: row.get(9)?,
            })
        })?;

        let mut instances = Vec::new();
        for inst in rows {
            instances.push(inst?);
        }
        Ok(instances)
    }

    pub fn get_instance(&self, id: &str) -> rusqlite::Result<Option<Instance>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, mc_version, loader, loader_version, ram_mb, game_dir, java_path, created_at, last_played_at
             FROM instances WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map(params![id], |row| {
            let game_dir_str: String = row.get(6)?;
            let java_path_str: Option<String> = row.get(7)?;

            Ok(Instance {
                id: row.get(0)?,
                name: row.get(1)?,
                mc_version: row.get(2)?,
                loader: row.get(3)?,
                loader_version: row.get(4)?,
                ram_mb: row.get(5)?,
                game_dir: PathBuf::from(game_dir_str),
                java_path: java_path_str.map(PathBuf::from),
                created_at: row.get(8)?,
                last_played_at: row.get(9)?,
            })
        })?;

        if let Some(first) = rows.next() {
            Ok(Some(first?))
        } else {
            Ok(None)
        }
    }

    pub fn create_instance(&self, inst: &Instance) -> rusqlite::Result<()> {
        let java_str = inst.java_path.as_ref().map(|p| p.to_string_lossy().to_string());
        self.conn.execute(
            "INSERT INTO instances (id, name, mc_version, loader, loader_version, ram_mb, game_dir, java_path, created_at, last_played_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                inst.id,
                inst.name,
                inst.mc_version,
                inst.loader,
                inst.loader_version,
                inst.ram_mb,
                inst.game_dir.to_string_lossy().to_string(),
                java_str,
                inst.created_at,
                inst.last_played_at,
            ],
        )?;
        Ok(())
    }

    pub fn update_last_played(&self, id: &str) -> rusqlite::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE instances SET last_played_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn delete_instance(&self, id: &str) -> rusqlite::Result<()> {
        self.conn.execute("DELETE FROM instances WHERE id = ?1", params![id])?;
        Ok(())
    }
}
