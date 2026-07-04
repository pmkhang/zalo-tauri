use gtk::glib::prelude::*;
use tauri::Manager;
use webkit2gtk::{NotificationExt, PermissionRequestExt, WebViewExt};

pub(super) fn install(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    let app_handle = window.app_handle().clone();

    window.with_webview(|platform_webview| {
        let webview = platform_webview.inner();

        webview.connect_permission_request(|_, request| {
            if request
                .downcast_ref::<webkit2gtk::NotificationPermissionRequest>()
                .is_some()
            {
                eprintln!("Đã cấp quyền thông báo cho Zalo Web");
                request.allow();
                true
            } else {
                false
            }
        });

        webview.connect_show_notification(move |_, notification| {
            show_native_notification(&app_handle, notification);
            true
        });
    })?;

    Ok(())
}

fn show_native_notification(
    app_handle: &tauri::AppHandle,
    notification: &webkit2gtk::Notification,
) {
    let title = notification
        .title()
        .map(|value| value.to_string())
        .unwrap_or_else(|| "Zalo".to_string());
    let body = notification
        .body()
        .map(|value| value.to_string())
        .unwrap_or_default();

    let result = notify_rust::Notification::new()
        .appname("Zalo")
        .summary(&title)
        .body(&body)
        .icon("zalo-tauri")
        .action("default", "Mở Zalo")
        .show();

    match result {
        Ok(handle) => wait_for_notification_action(app_handle.clone(), handle),
        Err(error) => eprintln!("Không thể hiển thị thông báo Zalo: {error}"),
    }
}

fn wait_for_notification_action(
    app_handle: tauri::AppHandle,
    handle: notify_rust::NotificationHandle,
) {
    std::thread::spawn(move || {
        handle.wait_for_action(move |action| {
            if action == "default" {
                open_main_window(app_handle.clone());
            }
        });
    });
}

fn open_main_window(app_handle: tauri::AppHandle) {
    let window_app_handle = app_handle.clone();
    if let Err(error) = app_handle.run_on_main_thread(move || {
        crate::window::show_main(&window_app_handle);
    }) {
        eprintln!("Không thể mở Zalo từ thông báo: {error}");
    }
}
