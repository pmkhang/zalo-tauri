use gtk::prelude::*;
use std::{cell::Cell, rc::Rc};

const TITLEBAR_CSS: &[u8] = include_bytes!("titlebar.css");

pub(super) fn install(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    let gtk_window = window.gtk_window()?;
    gtk_window.set_decorated(true);

    let (header, drag_area) = build_header(&gtk_window);
    install_drag_handlers(&gtk_window, &drag_area);
    install_styles();

    gtk_window.set_titlebar(Some(&header));
    header.show_all();
    Ok(())
}

fn build_header(gtk_window: &gtk::ApplicationWindow) -> (gtk::HeaderBar, gtk::EventBox) {
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
    header.pack_end(&build_window_controls(gtk_window));

    (header, drag_area)
}

fn build_window_controls(gtk_window: &gtk::ApplicationWindow) -> gtk::Box {
    let minimize = window_button("window-minimize-symbolic", "Thu nhỏ");
    let maximize = window_button("window-maximize-symbolic", "Phóng to / Khôi phục");
    let close = window_button("window-close-symbolic", "Đóng xuống khay hệ thống");
    close.style_context().add_class("zalo-close-button");

    let window = gtk_window.clone();
    minimize.connect_clicked(move |_| window.iconify());

    let window = gtk_window.clone();
    maximize.connect_clicked(move |_| toggle_maximized(&window));

    let window = gtk_window.clone();
    close.connect_clicked(move |_| window.hide());

    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    controls.style_context().add_class("zalo-window-controls");
    controls.pack_start(&minimize, false, false, 0);
    controls.pack_start(&maximize, false, false, 0);
    controls.pack_start(&close, false, false, 0);
    controls
}

fn window_button(icon: &str, tooltip: &str) -> gtk::Button {
    let button = gtk::Button::new();
    button.set_image(Some(&gtk::Image::from_icon_name(
        Some(icon),
        gtk::IconSize::Menu,
    )));
    button.set_tooltip_text(Some(tooltip));
    button.style_context().add_class("zalo-window-button");
    button.set_relief(gtk::ReliefStyle::None);
    button.set_focus_on_click(false);
    button
}

fn install_drag_handlers(gtk_window: &gtk::ApplicationWindow, drag_area: &gtk::EventBox) {
    let drag_origin = Rc::new(Cell::new(None::<(f64, f64)>));

    let origin = drag_origin.clone();
    let window = gtk_window.clone();
    drag_area.connect_button_press_event(move |_, event| {
        if event.button() != 1 {
            return gtk::glib::Propagation::Proceed;
        }

        if event.event_type() == gtk::gdk::EventType::DoubleButtonPress {
            origin.set(None);
            toggle_maximized(&window);
        } else {
            origin.set(Some(event.root()));
        }
        gtk::glib::Propagation::Stop
    });

    let origin = drag_origin.clone();
    let window = gtk_window.clone();
    drag_area.connect_motion_notify_event(move |_, event| {
        if event.state().contains(gtk::gdk::ModifierType::BUTTON1_MASK) {
            if let Some((start_x, start_y)) = origin.get() {
                let (x, y) = event.root();
                if (x - start_x).abs() > 4.0 || (y - start_y).abs() > 4.0 {
                    origin.set(None);
                    window.begin_move_drag(1, x as i32, y as i32, event.time());
                    return gtk::glib::Propagation::Stop;
                }
            }
        }
        gtk::glib::Propagation::Proceed
    });

    drag_area.connect_button_release_event(move |_, _| {
        drag_origin.set(None);
        gtk::glib::Propagation::Proceed
    });
}

fn toggle_maximized(window: &gtk::ApplicationWindow) {
    if window.is_maximized() {
        window.unmaximize();
    } else {
        window.maximize();
    }
}

fn install_styles() {
    let css = gtk::CssProvider::new();
    css.load_from_data(TITLEBAR_CSS)
        .expect("invalid titlebar CSS");

    if let Some(screen) = gtk::gdk::Screen::default() {
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
