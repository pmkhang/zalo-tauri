mod notifications;
mod titlebar;

pub(crate) fn setup_main_window(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    titlebar::install(window)?;
    notifications::install(window)?;
    Ok(())
}
