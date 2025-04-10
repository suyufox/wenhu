

/*
 * @Author: suyufox shenming26@outlook.com
 * @Date: 2025-04-07 20:02:20 +0800
 * @LastEditors: suyufox shenming26@outlook.com
 * @LastEditTime: 2025-04-10 19:56:45 +0800
 * @FilePath: \\wenhu\\src-tauri\\src\\lib.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */

use tauri::{AppHandle, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|wenhu, args, cwd| {
            println!("a new app instance was opened with {args:?} and the deep link event was already triggered");
            // when defining deep link schemes at runtime, you must also check `argv` here

            // 聚焦到主窗口
            let _ = wenhu.get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder
        // tauri 深度链接
        .plugin(tauri_plugin_deep_link::init())
        // tauri 剪切板
        .plugin(tauri_plugin_clipboard_manager::init())
        // tauri 对话弹窗
        .plugin(tauri_plugin_dialog::init())
        // tauri 文件系统
        .plugin(tauri_plugin_fs::init())
        // tauri 日志
        .plugin(tauri_plugin_log::Builder::new()
            // 清理默认架构
            .clear_targets()
            // 启用日志输出到终端
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Stdout,
            ))
            // 启用日志输出到 webview 控制台
            // 注意：这会导致日志输出到 webview 控制台的速度比终端慢，因为 webview 控制台是异步的
            // 并且需要以下代码：
            // ```js
            // import { attachConsole } from '@tauri-apps/plugin-log';
            // const detach = await attachConsole();
            // // call detach() if you do not want to print logs to the console anymore
            // ```
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Webview,
            ))
            // 日志持久化
            // 文件地址：
            // windows: | {FOLDERID_LocalAppData}/{bundleIdentifier}/logs
            //          | C:\Users\Alice\AppData\Local\com.tauri.dev\logs
            // linux:   | $XDG_DATA_HOME/{bundleIdentifier}/logs
            //          | $HOME/.local/share/{bundleIdentifier}/logs
            //          | /home/alice/.local/share/com.tauri.dev/logs
            // macOS:   | {homeDir}/Library/Logs/{bundleIdentifier}
            //          | /Users/Alice/Library/Logs/com.tauri.dev
            // ${identifier} 是应用的标识符，在 tauri.conf.json 中定义
            // 文件名称： ${file_name}.log
            // 或者使用 path: std::path::PathBuf::from("/path/to/logs"), 自定义日志文件路径
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("wenhu-later".to_string()),
                },
            ))
            // 日志格式化
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {}] {}",
                    record.level(),
                    record.target(),
                    message
                ))
            })
            // 日志时区
            .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
            // 日志文件大小限制
            // 默认：16 MB
            // 注意：这是每个日志文件的大小限制，而不是整个日志文件的总大小
            // 例如，如果您设置了 16 MB，那么当日志文件达到 16 MB 时，它将被重命名为 ${file_name}.1.log
            // 并且一个新的日志文件将被创建，继续写入日志
            .max_file_size(16_777_216 /* bytes */)
            // 日志文件保留
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            // 日志级别
            // 默认：LevelFilter::Info
            // 注意：这是日志的级别，而不是日志文件的级别
            // 例如，如果您设置了 LevelFilter::Info，那么所有级别为 Info、Warn、Error 的日志都会被写入日志文件
            // 但是，级别为 Debug 的日志将不会被写入日志
            .level(log::LevelFilter::Debug)
            .build())
        // tauri 通知
        .plugin(tauri_plugin_notification::init())
        // tauri open
        .plugin(tauri_plugin_opener::init())
        // tauri os信息
        .plugin(tauri_plugin_os::init())
        // tauri 持久化
        .plugin(tauri_plugin_persisted_scope::init())
        // tauri 进程
        .plugin(tauri_plugin_process::init())
        // tauri shell
        .plugin(tauri_plugin_shell::init())
        // tauri store
        .plugin(tauri_plugin_store::Builder::new().build())
        // https://github.com/HuakunShen/tauri-plugin-network
        // network
        .plugin(tauri_plugin_network::init())
        // https://github.com/HuakunShen/tauri-plugin-system-info
        // system info
        .plugin(tauri_plugin_system_info::init())
        // pinia 持久化
        // https://tb.dev.br/tauri-store/plugin-pinia/guide/getting-started
        // import { createApp } from 'vue';
        // import { createPinia } from 'pinia';
        // import { createPlugin } from '@tauri-store/pinia';
        //
        // const app = createApp(App);
        //
        // const pinia = createPinia();
        // pinia.use(createPlugin());
        //
        // app.use(pinia);
        // app.mount('#app');
        // 提示
        //
        // createPlugin 也导出为 TauriPluginPinia。
        // 定义
        // import { ref } from 'vue';
        // import { defineStore } from 'pinia';
        //
        // export const useStore = defineStore('counter', () => {
        //   const counter = ref(0);
        //   return { counter };
        // });
        // 启动、
        // const store = useStore();
        // await store.$tauri.start();
        // 提示
        //
        // 在您调用 start 方法之前，不会保存或同步 store。
        .plugin(tauri_plugin_pinia::Builder::new()
            .autosave(std::time::Duration::from_secs(300))
            .build())
        .setup(|wenhu| {
            #[cfg(any(windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                wenhu.deep_link().register_all()?;
            }
            Ok(())
        })
        .setup(|wenhu| {
            // tauri 快捷键
            #[cfg(desktop)]
            wenhu.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build());
            Ok(())
        })
        // tauri stronghold
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        // tauri 窗口状态
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_window_state::Builder::default().build());
            Ok(())
        })
        // .setup(|wenhu| {
        //     #[cfg(desktop)]
        //     wenhu.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build());
        //     wenhu.handle().plugin(tauri_plugin_window_state::Builder::default().build());
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application | 运行应用时发生错误");
}






