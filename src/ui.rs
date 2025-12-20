use std::rc::Rc;

use gtk4::{
    Application, CssProvider,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
};

use crate::options::Options;

fn build_ui(app: &Application, opts: &Options) {}

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

pub fn run_ui(opts: Rc<Options>) {
    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    let opts_clone = opts.clone();
    app.connect_startup(move |_| load_css(&opts_clone));

    app.connect_activate(move |app| {
        build_ui(app, &opts);
    });

    app.run_with_args::<String>(&[]);
}
