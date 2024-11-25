extern crate core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_stronghold::Builder::new(|password| {
            use argon2::{hash_raw, Config, Variant, Version};

            let config = Config {
                lanes: 4,
                mem_cost: 10_000,
                time_cost: 10,
                variant: Variant::Argon2d,
                version: Version::Version10,
                ..Default::default()
            };
            let salt = "VVXg7vmsM".as_bytes();
            let key = hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

            key.to_vec()
        }).build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(|out, message, record| {
                    let format = time::format_description::parse(
                        "[[[year]-[month]-[day]][[[hour]:[minute]:[second]]",
                    )
                    .unwrap();
                    let now = tauri_plugin_log::TimezoneStrategy::UseLocal
                        .get_now()
                        .format(&format)
                        .unwrap();
                    out.finish(format_args!("{}[{}] {}", now, record.level(), message))
                })
                .max_file_size(10 * 1_000_000 /* 10MB */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_power_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, commands::check_connection, commands::open_inspect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

