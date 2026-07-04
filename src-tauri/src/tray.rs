use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

pub(crate) fn install(app: &tauri::App) -> tauri::Result<()> {
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
            "open" => crate::window::show_main(app),
            "hide" => crate::window::hide_main(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}
