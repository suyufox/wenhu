#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    builder
        // .plugin(tauri_plugin_wenhu_core::init())
        // .plugin(tauri_plugin_wenhu_configs::init())
        // .plugin(tauri_plugin_wenhu_plugins::init())
        // .plugin(tauri_plugin_wenhu_server::init())
        // .plugin(tauri_plugin_wenhu_updater::init())
        // .plugin(tauri_plugin_fs::init())
        // .plugin(tauri_plugin_system_info::init())
        // .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_opener::init())
        // .plugin(tauri_plugin_os::init())
        // .plugin(tauri_plugin_process::init())
        // .plugin(tauri_plugin_persisted_scope::init())
        // .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_log::Builder
                ::new()
                .clear_targets()
                .format(|out, message, record| {
                    out.finish(format_args!("[{} {}] {}", record.level(), record.target(), message))
                })
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview))
                .target(
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("wenhu-laster".to_string()),
                    })
                )
                .max_file_size(25_165_824 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build()
        )
        // .setup(|wenhu| {
        //     #[cfg(desktop)]
        //     wenhu.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build());
        //     wenhu.handle().plugin(tauri_plugin_window_state::Builder::default().build());
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application | 运行应用时发生错误");
}
