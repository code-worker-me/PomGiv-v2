mod db;

use std::sync::Mutex;
use tauri::{LogicalSize, Manager, Size};
use tauri_plugin_notification::NotificationExt;

pub struct DbState(pub Mutex<rusqlite::Connection>);
pub struct MetricsState(pub Mutex<sysinfo::System>);

#[derive(serde::Serialize, Clone, Debug)]
pub struct AppMetrics {
    pub cpu_usage: f32,
    pub memory_used_mb: f64,
    pub total_system_memory_mb: u64,
}

#[tauri::command]
fn get_timer_settings(state: tauri::State<DbState>) -> Result<db::TimerSettings, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_settings(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_timer_settings(
    state: tauri::State<DbState>,
    hours: u32,
    minutes: u32,
    seconds: u32,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::save_settings(&conn, hours, minutes, seconds).map_err(|e| e.to_string())
}

#[tauri::command]
fn complete_study_session(
    app: tauri::AppHandle,
    state: tauri::State<DbState>,
    duration_seconds: u32,
) -> Result<bool, String> {
    // Validasi ketat: HANYA jika durasi lebih dari 1 menit (> 60 detik)
    if duration_seconds <= 60 {
        return Ok(false);
    }

    // 1. Simpan ke SQLite
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::log_study_session(&conn, duration_seconds).map_err(|e| e.to_string())?;

    // 2. Munculkan notifikasi Windows
    let minutes = duration_seconds / 60;
    let _ = app.notification()
        .builder()
        .title("Waktu Belajar Selesai! 🎉")
        .body(format!("Selamat! Sesi belajar selama {} menit telah selesai.", minutes))
        .show();

    Ok(true)
}

#[tauri::command]
fn get_monthly_study_stats(
    state: tauri::State<DbState>,
    year: Option<u32>,
    month: Option<u32>,
) -> Result<db::MonthlyStudySummary, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_monthly_stats(&conn, year, month).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_mini_mode(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if enabled {
            window.set_always_on_top(true).map_err(|e| e.to_string())?;
            window
                .set_min_size(Some(Size::Logical(LogicalSize::new(240.0, 90.0))))
                .map_err(|e| e.to_string())?;
            window
                .set_size(Size::Logical(LogicalSize::new(300.0, 115.0)))
                .map_err(|e| e.to_string())?;
        } else {
            window.set_always_on_top(false).map_err(|e| e.to_string())?;
            window
                .set_min_size(Some(Size::Logical(LogicalSize::new(460.0, 600.0))))
                .map_err(|e| e.to_string())?;
            window
                .set_size(Size::Logical(LogicalSize::new(460.0, 600.0)))
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
fn get_app_metrics(state: tauri::State<MetricsState>) -> Result<AppMetrics, String> {
    let mut sys = state.0.lock().map_err(|e| e.to_string())?;
    let pid = sysinfo::Pid::from_u32(std::process::id());

    // Refresh metrik proses PomGiv & memori sistem
    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
    sys.refresh_memory();

    let (cpu_usage, memory_used_mb) = if let Some(proc) = sys.process(pid) {
        let mem = (proc.memory() as f64) / (1024.0 * 1024.0);
        let cpu = proc.cpu_usage();
        (cpu, mem)
    } else {
        (0.0, 0.0)
    };

    let total_system_memory_mb = sys.total_memory() / (1024 * 1024);

    Ok(AppMetrics {
        cpu_usage,
        memory_used_mb: (memory_used_mb * 100.0).round() / 100.0,
        total_system_memory_mb,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let conn = db::init_db(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
            app.manage(DbState(Mutex::new(conn)));

            // Inisialisasi pengukur metrik (sysinfo)
            let mut sys = sysinfo::System::new();
            let pid = sysinfo::Pid::from_u32(std::process::id());
            sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
            app.manage(MetricsState(Mutex::new(sys)));

            // Logger metrik otomatis di terminal saat development mode (npm run tauri dev)
            #[cfg(debug_assertions)]
            {
                std::thread::spawn(move || {
                    use std::time::Duration;
                    let mut monitor_sys = sysinfo::System::new();
                    let target_pid = sysinfo::Pid::from_u32(std::process::id());

                    loop {
                        std::thread::sleep(Duration::from_secs(3));
                        monitor_sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[target_pid]), true);
                        if let Some(proc) = monitor_sys.process(target_pid) {
                            let mem_mb = (proc.memory() as f64) / (1024.0 * 1024.0);
                            let cpu = proc.cpu_usage();
                            println!(
                                "\x1b[35m[PomGiv Dev Metrics]\x1b[0m CPU: {:>5.1}% | RAM: {:>6.2} MB",
                                cpu, mem_mb
                            );
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_timer_settings,
            save_timer_settings,
            complete_study_session,
            get_monthly_study_stats,
            set_mini_mode,
            get_app_metrics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


