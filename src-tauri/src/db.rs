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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: i64,
    pub title: String,
    pub is_checked: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivitySummaryItem {
    pub title: String,
    pub duration_seconds: u32,
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

    conn.execute(
        "CREATE TABLE IF NOT EXISTS study_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            duration_seconds INTEGER NOT NULL,
            completed_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Migrasi aman: Tambah kolom activity_title jika belum ada di tabel study_sessions
    let has_activity_title = {
        let mut stmt = conn.prepare("PRAGMA table_info(study_sessions)")?;
        let column_rows = stmt.query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?;
        let mut found = false;
        for col_name in column_rows.flatten() {
            if col_name == "activity_title" {
                found = true;
                break;
            }
        }
        found
    };

    if !has_activity_title {
        conn.execute("ALTER TABLE study_sessions ADD COLUMN activity_title TEXT", [])?;
    }

    // Tabel untuk menyimpan daftar aktivitas harian
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            is_checked INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
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

pub fn log_study_session(
    conn: &Connection,
    duration_seconds: u32,
    activity_title: Option<String>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO study_sessions (duration_seconds, activity_title, completed_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
        params![duration_seconds, activity_title],
    )?;

    Ok(())
}

pub fn get_activities(conn: &Connection) -> Result<Vec<Activity>> {
    let mut stmt = conn.prepare("SELECT id, title, is_checked FROM activities ORDER BY id ASC")?;
    let rows = stmt.query_map([], |row| {
        let is_checked_int: i64 = row.get(2)?;
        Ok(Activity {
            id: row.get(0)?,
            title: row.get(1)?,
            is_checked: is_checked_int != 0,
        })
    })?;

    let mut list = Vec::new();
    for item in rows {
        list.push(item?);
    }
    Ok(list)
}

pub fn add_activity(conn: &Connection, title: &str) -> Result<Activity> {
    conn.execute(
        "INSERT INTO activities (title, is_checked) VALUES (?1, 0)",
        params![title],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Activity {
        id,
        title: title.to_string(),
        is_checked: false,
    })
}

pub fn update_activity_checked(conn: &Connection, id: i64, is_checked: bool) -> Result<()> {
    let val: i64 = if is_checked { 1 } else { 0 };
    conn.execute(
        "UPDATE activities SET is_checked = ?1 WHERE id = ?2",
        params![val, id],
    )?;
    Ok(())
}

pub fn delete_activity(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM activities WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_daily_activity_summary(
    conn: &Connection,
    year: u32,
    month: u32,
    day: u32,
) -> Result<Vec<ActivitySummaryItem>> {
    let date_str = format!("{}-{:02}-{:02}", year, month, day);
    let mut stmt = conn.prepare(
        "SELECT activity_title, SUM(duration_seconds) as total_seconds
         FROM study_sessions
         WHERE strftime('%Y-%m-%d', completed_at, 'localtime') = ?1
           AND activity_title IS NOT NULL
           AND TRIM(activity_title) != ''
         GROUP BY activity_title
         ORDER BY total_seconds DESC",
    )?;

    let rows = stmt.query_map(params![date_str], |row| {
        Ok(ActivitySummaryItem {
            title: row.get(0)?,
            duration_seconds: row.get(1)?,
        })
    })?;

    let mut list = Vec::new();
    for item in rows {
        list.push(item?);
    }
    Ok(list)
}

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyStudyStat {
    pub day: u32,
    pub minutes: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonthlyStudySummary {
    pub year: u32,
    pub month: u32,
    pub days_in_month: u32,
    pub today_day: u32,
    pub today_minutes: u32,
    pub total_month_minutes: u32,
    pub daily_stats: Vec<DailyStudyStat>,
}

pub fn get_monthly_stats(
    conn: &Connection,
    year_opt: Option<u32>,
    month_opt: Option<u32>,
) -> Result<MonthlyStudySummary> {
    // 1. Ambil tahun, bulan, dan hari ini dari SQLite
    let (cur_year, cur_month, cur_day): (u32, u32, u32) = conn.query_row(
        "SELECT 
            CAST(strftime('%Y', 'now', 'localtime') AS INTEGER),
            CAST(strftime('%m', 'now', 'localtime') AS INTEGER),
            CAST(strftime('%d', 'now', 'localtime') AS INTEGER)",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;

    let year = year_opt.unwrap_or(cur_year);
    let month = month_opt.unwrap_or(cur_month);

    // 2. Hitung jumlah hari dalam bulan tersebut
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    };

    // 3. Query total durasi (detik) per hari pada bulan tersebut
    let year_month_str = format!("{}-{:02}", year, month);
    let mut stmt = conn.prepare(
        "SELECT 
            CAST(strftime('%d', completed_at, 'localtime') AS INTEGER) as day,
            SUM(duration_seconds) as total_seconds
         FROM study_sessions
         WHERE strftime('%Y-%m', completed_at, 'localtime') = ?1
         GROUP BY day",
    )?;

    let mut day_seconds_map: HashMap<u32, u32> = HashMap::new();
    let rows = stmt.query_map(params![year_month_str], |row| {
        let day: u32 = row.get(0)?;
        let seconds: u32 = row.get(1)?;
        Ok((day, seconds))
    })?;

    for item in rows {
        if let Ok((d, s)) = item {
            day_seconds_map.insert(d, s);
        }
    }

    // 4. Bangun daily_stats dari hari ke-1 sampai hari terakhir bulan
    let mut daily_stats = Vec::with_capacity(days_in_month as usize);
    let mut total_month_minutes = 0u32;
    let mut today_minutes = 0u32;

    for day in 1..=days_in_month {
        let total_secs = day_seconds_map.get(&day).copied().unwrap_or(0);
        let mins = total_secs / 60;
        daily_stats.push(DailyStudyStat {
            day,
            minutes: mins,
        });

        total_month_minutes += mins;

        if year == cur_year && month == cur_month && day == cur_day {
            today_minutes = mins;
        }
    }

    Ok(MonthlyStudySummary {
        year,
        month,
        days_in_month,
        today_day: cur_day,
        today_minutes,
        total_month_minutes,
        daily_stats,
    })
}


