use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, WindowEvent,
};

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use std::{cell::Cell, rc::Rc};

#[cfg(target_os = "linux")]
fn install_compact_titlebar(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    let gtk_window = window.gtk_window()?;
    gtk_window.set_decorated(true);
    let header = gtk::HeaderBar::new();
    header.set_show_close_button(false);
    header.style_context().add_class("zalo-compact-titlebar");

    let title = gtk::Label::new(Some("Zalo"));
    title.style_context().add_class("zalo-title");
    let drag_area = gtk::EventBox::new();
    drag_area.set_visible_window(false);
    drag_area.set_hexpand(true);
    drag_area.add_events(
        gtk::gdk::EventMask::BUTTON_PRESS_MASK
            | gtk::gdk::EventMask::BUTTON_RELEASE_MASK
            | gtk::gdk::EventMask::BUTTON_MOTION_MASK,
    );
    drag_area.add(&title);
    header.set_custom_title(Some(&drag_area));

    let minimize = gtk::Button::new();
    let maximize = gtk::Button::new();
    let close = gtk::Button::new();
    minimize.set_image(Some(&gtk::Image::from_icon_name(
        Some("window-minimize-symbolic"),
        gtk::IconSize::Menu,
    )));
    maximize.set_image(Some(&gtk::Image::from_icon_name(
        Some("window-maximize-symbolic"),
        gtk::IconSize::Menu,
    )));
    close.set_image(Some(&gtk::Image::from_icon_name(
        Some("window-close-symbolic"),
        gtk::IconSize::Menu,
    )));
    minimize.set_tooltip_text(Some("Thu nhỏ"));
    maximize.set_tooltip_text(Some("Phóng to / Khôi phục"));
    close.set_tooltip_text(Some("Đóng xuống khay hệ thống"));
    for button in [&minimize, &maximize, &close] {
        button.style_context().add_class("zalo-window-button");
        button.set_relief(gtk::ReliefStyle::None);
        button.set_focus_on_click(false);
    }
    close.style_context().add_class("zalo-close-button");

    let win = gtk_window.clone();
    minimize.connect_clicked(move |_| win.iconify());
    let win = gtk_window.clone();
    maximize.connect_clicked(move |_| {
        if win.is_maximized() {
            win.unmaximize();
        } else {
            win.maximize();
        }
    });
    let win = gtk_window.clone();
    close.connect_clicked(move |_| win.hide());

    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    controls.style_context().add_class("zalo-window-controls");
    controls.pack_start(&minimize, false, false, 0);
    controls.pack_start(&maximize, false, false, 0);
    controls.pack_start(&close, false, false, 0);
    header.pack_end(&controls);

    let drag_origin = Rc::new(Cell::new(None::<(f64, f64)>));
    let origin = drag_origin.clone();
    let win = gtk_window.clone();
    drag_area.connect_button_press_event(move |_, event| {
        if event.button() == 1 {
            if event.event_type() == gtk::gdk::EventType::DoubleButtonPress {
                origin.set(None);
                if win.is_maximized() {
                    win.unmaximize();
                } else {
                    win.maximize();
                }
            } else {
                origin.set(Some(event.root()));
            }
            return gtk::glib::Propagation::Stop;
        }
        gtk::glib::Propagation::Proceed
    });

    let origin = drag_origin.clone();
    let win = gtk_window.clone();
    drag_area.connect_motion_notify_event(move |_, event| {
        if event.state().contains(gtk::gdk::ModifierType::BUTTON1_MASK) {
            if let Some((start_x, start_y)) = origin.get() {
                let (x, y) = event.root();
                if (x - start_x).abs() > 4.0 || (y - start_y).abs() > 4.0 {
                    origin.set(None);
                    win.begin_move_drag(1, x as i32, y as i32, event.time());
                    return gtk::glib::Propagation::Stop;
                }
            }
        }
        gtk::glib::Propagation::Proceed
    });

    let origin = drag_origin;
    drag_area.connect_button_release_event(move |_, _| {
        origin.set(None);
        gtk::glib::Propagation::Proceed
    });

    let css = gtk::CssProvider::new();
    css.load_from_data(
        b"headerbar.zalo-compact-titlebar {
            min-height: 24px;
            padding: 0 3px;
            margin: 0;
            border-radius: 0;
          }
          headerbar.zalo-compact-titlebar .zalo-title {
            font-size: 11px;
            font-weight: 500;
            padding: 0;
            margin: 0;
          }
          headerbar.zalo-compact-titlebar .zalo-window-button {
            min-width: 28px;
            min-height: 20px;
            padding: 0;
            margin: 1px;
            border: 0;
            border-radius: 5px;
            background: transparent;
            box-shadow: none;
            transition: 120ms ease-out;
          }
          headerbar.zalo-compact-titlebar .zalo-window-button:hover {
            background: alpha(currentColor, 0.10);
          }
          headerbar.zalo-compact-titlebar .zalo-window-button:active {
            background: alpha(currentColor, 0.17);
          }
          headerbar.zalo-compact-titlebar .zalo-window-button image {
            -gtk-icon-transform: scale(0.78);
          }
          headerbar.zalo-compact-titlebar .zalo-close-button:hover {
            background: #e5484d;
            color: white;
          }
          headerbar.zalo-compact-titlebar .zalo-close-button:active {
            background: #c9363e;
            color: white;
          }",
    )
    .expect("invalid titlebar CSS");

    if let Some(screen) = gtk::gdk::Screen::default() {
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    gtk_window.set_titlebar(Some(&header));
    header.show_all();
    Ok(())
}

#[cfg(target_os = "linux")]
fn install_web_notifications(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    let app_handle = window.app_handle().clone();
    window.with_webview(|platform_webview| {
        use gtk::glib::prelude::*;
        use webkit2gtk::{NotificationExt, PermissionRequestExt, WebViewExt};

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
            let title = notification
                .title()
                .map(|value| value.to_string())
                .unwrap_or_else(|| "Zalo".to_string());
            let body = notification
                .body()
                .map(|value| value.to_string())
                .unwrap_or_default();

            // gio::Application::send_notification silently does nothing when the
            // GTK application has no registered application ID. Tauri's Linux
            // runtime does not guarantee that registration, so talk to the
            // desktop notification service directly instead.
            let result = notify_rust::Notification::new()
                .appname("Zalo")
                .summary(&title)
                .body(&body)
                .icon("zalo-tauri")
                .action("default", "Mở Zalo")
                .show();

            match result {
                Ok(handle) => {
                    let app_handle = app_handle.clone();
                    std::thread::spawn(move || {
                        handle.wait_for_action(move |action| {
                            if action == "default" {
                                let window_app_handle = app_handle.clone();
                                if let Err(error) = app_handle.run_on_main_thread(move || {
                                    show_main_window(&window_app_handle);
                                }) {
                                    eprintln!("Không thể mở Zalo từ thông báo: {error}");
                                }
                            }
                        });
                    });
                }
                Err(error) => eprintln!("Không thể hiển thị thông báo Zalo: {error}"),
            }
            true
        });
    })?;
    Ok(())
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "linux")]
                {
                    install_compact_titlebar(&window)?;
                    install_web_notifications(&window)?;
                }
            }

            let open = MenuItem::with_id(app, "open", "Mở Zalo", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Ẩn Zalo", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Thoát hoàn toàn", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open, &hide, &quit])?;

            TrayIconBuilder::with_id("zalo-tray")
                .icon(app.default_window_icon().expect("missing app icon").clone())
                .tooltip("Zalo")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => show_main_window(app),
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run Zalo");
}
