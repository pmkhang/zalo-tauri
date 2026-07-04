use tauri::Manager;

pub(crate) fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            crate::window::show_main(app);
        }))
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                crate::linux::setup_main_window(&window)?;
            }

            crate::tray::install(app)?;
            Ok(())
        })
        .on_window_event(crate::window::handle_event)
        .run(tauri::generate_context!())
        .expect("failed to run Zalo");
}
