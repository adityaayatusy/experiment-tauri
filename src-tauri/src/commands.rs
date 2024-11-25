use std::time::Duration;
use tauri::command;
cargo add tauri-plugin-context-menuuse tokio::{process::Command, time::sleep};

#[command]
pub async fn check_connection(app: AppHandle, domain: String) {
    loop {
        match Command::new("ping")
            .args(["-c", "1", &domain])
            .output()
            .await
        {
            Ok(output) => {
                let success = output.status.success();
                if let Err(e) = app.emit("ping_result", success) {
                    eprintln!("Failed to emit ping result: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute ping: {}", e);
                if let Err(e) = app.emit("ping_result", false) {
                    eprintln!("Failed to emit ping error result: {}", e);
                }
            }
        }

        // Tunggu 1 detik sebelum mengulangi pengecekan
        sleep(Duration::from_secs(1)).await;
    }
}


#[command]
pub async fn open_inspect(app: AppHandle) {
    use tauri::Manager;
    let window = app.get_webview_window("main").unwrap();
    window.open_devtools();
}