use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use gtk4::{
    Application, CssProvider,
    gdk::prelude::{DisplayExt, MonitorExt},
    gio::prelude::{ApplicationExt, ApplicationExtManual, ListModelExt},
    glib::object::Cast,
    prelude::{FixedExt, GtkWindowExt, WidgetExt},
};

use crate::{options::Options, sway};

fn calculate_geometry(
    window: &swayipc::Node,
    output: &swayipc::Node,
    opts: &Options,
) -> (i32, i32) {
    let rect = window.rect;
    let window_rect = window.window_rect;
    let deco_rect = window.deco_rect;

    let anchor_x = output.rect.x;
    let anchor_y = output.rect.y;

    let rel_x = rect.x + window_rect.x + deco_rect.x + opts.label_margin_x;
    let rel_y = rect.y - (deco_rect.height - opts.label_margin_y)
        + if window.layout == swayipc::NodeLayout::Stacked {
            deco_rect.y
        } else {
            0
        };

    (rel_x - anchor_x, rel_y - anchor_y)
}

fn build_ui(
    app: &Application,
    conn: &Rc<RefCell<swayipc::Connection>>,
    opts: &Options,
) -> Result<(), Error> {
    let mut conn = conn.try_borrow_mut().map_err(|_| Error::ConnectionError)?;
    let tree = conn.get_tree()?;

    let outputs = sway::parse_output_nodes(&tree);
    let mut chars = opts.chars.chars();

    let mut keys_to_con_ids = HashMap::new();
    let mut window_ids_to_data = HashMap::new();
    let mut windows = Vec::new();

    for output in outputs {
        let window = gtk4::ApplicationWindow::new(app);

        // Configure layer shell.
        // Setting a namespace allows WM rules to target these windows.
        gtk4_layer_shell::LayerShell::init_layer_shell(&window);
        gtk4_layer_shell::LayerShell::set_namespace(&window, Some("sway-easyfocus"));
        gtk4_layer_shell::LayerShell::set_layer(&window, gtk4_layer_shell::Layer::Overlay);
        gtk4_layer_shell::LayerShell::set_keyboard_mode(
            &window,
            gtk4_layer_shell::KeyboardMode::Exclusive,
        );
        gtk4_layer_shell::LayerShell::set_anchor(&window, gtk4_layer_shell::Edge::Top, true);
        gtk4_layer_shell::LayerShell::set_anchor(&window, gtk4_layer_shell::Edge::Bottom, true);
        gtk4_layer_shell::LayerShell::set_anchor(&window, gtk4_layer_shell::Edge::Left, true);
        gtk4_layer_shell::LayerShell::set_anchor(&window, gtk4_layer_shell::Edge::Right, true);

        // Set monitor for this output.
        // This is necessary because Sway outputs (logical displays in the window manager) need to
        // be mapped to GTK/GDK monitors (physical displays as seen by GTK) so that the overlay
        // labels appear on the correct physical screen in a multi-monitor setup.
        let display = gtk4::gdk::Display::default().unwrap();
        let monitors = display.monitors();
        for i in 0..monitors.n_items() {
            if let Some(monitor) = monitors
                .item(i)
                .and_then(|obj| obj.downcast::<gtk4::gdk::Monitor>().ok())
            {
                let geometry = monitor.geometry();
                if geometry.x() <= output.rect.x
                    && output.rect.x < geometry.x() + geometry.width()
                    && geometry.y() <= output.rect.y
                    && output.rect.y < geometry.y() + geometry.height()
                {
                    gtk4_layer_shell::LayerShell::set_monitor(&window, Some(&monitor));
                    break;
                }
            }
        }

        let fixed = gtk4::Fixed::new();

        if let Some(workspace) = sway::find_focused_workspace(output) {
            let client_windows = sway::get_all_windows(&workspace);

            // Create labels for windows
            for client in client_windows.iter() {
                let (x, y) = calculate_geometry(client, &output, opts);
                let label = gtk4::Label::new(Some(""));

                let letter = chars.next().ok_or(Error::OutOfCharsError)?;

                label.set_markup(&format!("{}", letter));

                label.set_halign(gtk4::Align::Center);
                label.set_valign(gtk4::Align::Center);

                fixed.put(&label, x as f64, y as f64);

                if client.focused {
                    label.add_css_class("focused");
                }

                // Store mappings
                keys_to_con_ids.insert(letter, client.id);
                window_ids_to_data.insert(client.id, (client.clone(), output.clone(), letter));
            }
        }

        window.set_child(Some(&fixed));
        windows.push(window);
    }

    for window in windows.iter() {
        window.present();
    }

    Ok(())
}

fn load_css(opts: &Options) {
    let provider = CssProvider::new();
    provider.load_from_data(&opts.to_css());

    // Add the provider to the default display
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn run_ui(conn: swayipc::Connection, opts: Rc<Options>) {
    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    let opts_clone = opts.clone();
    app.connect_startup(move |_| load_css(&opts_clone));

    let conn = Rc::new(RefCell::new(conn));
    app.connect_activate(move |app| {
        if let Err(err) = build_ui(app, &conn, &opts) {
            eprintln!("{}", err);
        }
    });

    app.run_with_args::<String>(&[]);
}

#[derive(Debug)]
pub enum Error {
    ConnectionError,
    SwayIpcError(swayipc::Error),
    OutOfCharsError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionError => f.write_str("An error occured with the connection."),
            Self::SwayIpcError(err) => f.write_fmt(format_args!("{}", err)),
            Self::OutOfCharsError => f.write_str("Ran out of character labels."),
        }
    }
}

impl From<swayipc::Error> for Error {
    fn from(value: swayipc::Error) -> Self {
        Self::SwayIpcError(value)
    }
}
