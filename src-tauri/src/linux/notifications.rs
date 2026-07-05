use gtk::glib::prelude::*;
use tauri::Manager;
use webkit2gtk::{
    NotificationExt, PermissionRequestExt, SecurityOrigin, UserContentInjectedFrames,
    UserContentManagerExt, UserScript, UserScriptInjectionTime, WebContextExt, WebViewExt,
};

const RESTORE_PERMISSION_SCRIPT: &str = include_str!("restore_notification_permission.js");

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

        initialize_notification_permission(&webview);
        install_permission_restore_script(&webview);
        webview.reload();
    })?;

    Ok(())
}

fn initialize_notification_permission(webview: &webkit2gtk::WebView) {
    if let Some(context) = webview.context() {
        context.connect_initialize_notification_permissions(|context| {
            allow_zalo_notification_origin(context);
        });
        allow_zalo_notification_origin(&context);
    } else {
        eprintln!("Không thể khôi phục quyền thông báo: thiếu WebKit context");
    }
}

fn install_permission_restore_script(webview: &webkit2gtk::WebView) {
    let script = UserScript::new(
        RESTORE_PERMISSION_SCRIPT,
        UserContentInjectedFrames::TopFrame,
        UserScriptInjectionTime::Start,
        &[],
        &[],
    );

    if let Some(content_manager) = webview.user_content_manager() {
        content_manager.add_script(&script);
    } else {
        eprintln!("Không thể khôi phục quyền thông báo: thiếu WebKit content manager");
    }
}

fn allow_zalo_notification_origin(context: &webkit2gtk::WebContext) {
    let origin = SecurityOrigin::for_uri("https://chat.zalo.me");
    context.initialize_notification_permissions(&[&origin], &[]);
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
