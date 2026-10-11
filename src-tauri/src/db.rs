use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimerSettings {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection, Box<dyn std::error::Error>> {
    let app_dir: PathBuf = app_handle.path().app_data_dir()?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }

    let db_path = app_dir.join("pomgiv.db");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS timer_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            hours INTEGER NOT NULL DEFAULT 0,
            minutes INTEGER NOT NULL DEFAULT 25,
            seconds INTEGER NOT NULL DEFAULT 0,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO timer_settings (id, hours, minutes, seconds) VALUES (1, 0, 25, 0)",
        [],
    )?;

    Ok(conn)
}

pub fn get_settings(conn: &Connection) -> Result<TimerSettings> {
    let mut stmt = conn.prepare("SELECT hours, minutes, seconds FROM timer_settings WHERE id = 1")?;
    let settings = stmt.query_row([], |row| {
        Ok(TimerSettings {
            hours: row.get(0)?,
            minutes: row.get(1)?,
            seconds: row.get(2)?,
        })
    })?;

    Ok(settings)
}

pub fn save_settings(conn: &Connection, hours: u32, minutes: u32, seconds: u32) -> Result<()> {
    conn.execute(
        "INSERT INTO timer_settings (id, hours, minutes, seconds, updated_at)
         VALUES (1, ?1, ?2, ?3, CURRENT_TIMESTAMP)
         ON CONFLICT(id) DO UPDATE SET
            hours = excluded.hours,
            minutes = excluded.minutes,
            seconds = excluded.seconds,
            updated_at = CURRENT_TIMESTAMP",
        params![hours, minutes, seconds],
    )?;

    Ok(())
}
