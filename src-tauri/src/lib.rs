mod brew;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            brew::list,
            brew::search,
            brew::install,
            brew::uninstall,
            brew::reinstall
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
