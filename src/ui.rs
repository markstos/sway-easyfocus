use std::sync::{Arc, Mutex};

use gtk::{prelude::*, Application, CssProvider, StyleContext};
use swayipc::{Connection, Node};

use crate::{cli::Args, sway, utils};

fn calculate_geometry(window: &Node, output: &Node, args: Arc<Args>) -> (i32, i32) {
    // TODO: this doesn't work properly with stacked windows
    let rect = window.rect;
    let window_rect = window.window_rect;
    let deco_rect = window.deco_rect;

    let anchor_x = output.rect.x;
    let anchor_y = output.rect.y;

    let rel_x = rect.x + window_rect.x + deco_rect.x + args.label_margin_x.unwrap();
    let rel_y = rect.y - (deco_rect.height - args.label_margin_y.unwrap());

    (rel_x - anchor_x, rel_y - anchor_y)
}

fn handle_keypress(conn: Arc<Mutex<Connection>>, windows: &[Node], keyval: &str) {
    if keyval.len() == 1 {
        // we can unwrap because the keyval has one character
        let c = keyval.chars().next().unwrap();
        if c.is_alphabetic() && c.is_lowercase() {
            // this is kinda hacky
            let c_index = c as usize - 'a' as usize;
            if c_index < windows.len() {
                sway::focus(conn, windows, c_index);
            }
        }
    }
}

fn build_ui(app: &Application, args: Arc<Args>, conn: Arc<Mutex<Connection>>) {
    // get windows from sway
    let output = sway::get_focused_output(conn.clone());
    let workspace = sway::get_focused_workspace(&output);
    let windows = sway::get_all_windows(&workspace);

    // exit if no windows open
    if windows.len() == 0 {
        return;
    }

    let window = gtk::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);
    // display it above normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // receive keyboard events from the compositor
    gtk_layer_shell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);

    let windows_clone = windows.clone();
    window.connect_key_press_event(move |window, event| {
        let keyval = event
            .keyval()
            .name()
            .expect("the key pressed does not have a name?");
        handle_keypress(conn.clone(), &windows_clone, &keyval);
        window.close();
        Inhibit(false)
    });

    // take up the full screen
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

    let fixed = gtk::Fixed::new();

    for (idx, window) in windows.iter().enumerate() {
        let (x, y) = calculate_geometry(window, &output, args.clone());
        let label = gtk::Label::new(Some(""));
        // TODO: make this work for workspaces with more than 26 windows
        label.set_markup(&format!("{}", ('a' as usize + idx % 26) as u8 as char));
        fixed.put(&label, x, y);

        // Apply a CSS class to the focused window so it can be styled differently
        if window.focused {
            label.style_context().add_class("focused");
        }
    }

    window.add(&fixed);

    window.show_all();
}

fn load_css(args: Arc<Args>) {
    let provider = CssProvider::new();
    provider
        .load_from_data(utils::args_to_css(&args).as_bytes())
        .expect("failed to load css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        // we can unwrap because there should be a default screen
        &gtk::gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn run_ui(conn: Arc<Mutex<Connection>>, args: Arc<Args>) {
    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    let args_clone = args.clone();
    app.connect_startup(move |_| load_css(args_clone.clone()));
    app.connect_activate(move |app| {
        build_ui(app, args.clone(), conn.clone());
    });

    let empty: Vec<String> = vec![];
    app.run_with_args(&empty);
}
