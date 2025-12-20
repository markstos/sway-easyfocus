use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use clap::Parser;
use figment::Figment;
use figment::providers::{Format, Yaml};

use crate::options::Options;

mod cli;
mod options;
mod sway;
mod ui;
mod util;

struct Lockfile {
    path: PathBuf,
}

impl Drop for Lockfile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path).expect(&format!(
            "Failed to remove lockfile at {}.",
            &self.path.display()
        ));
    }
}

impl Lockfile {
    pub fn new() -> Self {
        use std::env;

        let mut lock_path = env::var("TMPDIR").map_or(PathBuf::from("/tmp"), |dir| {
            let tentative_dir = Path::new(&dir);
            if tentative_dir.is_dir() {
                PathBuf::from(tentative_dir)
            } else {
                eprintln!("[$TMPDIR] is not a directory! Falling back to [/tmp].");
                PathBuf::from("/tmp")
            }
        });

        lock_path.push("sway-easyfocus-lockfile");
        File::create_new(&lock_path)
            .map(move |_| Self { path: lock_path })
            .expect("Lockfile exists! (Is another instance running?)")
    }
}

fn read_options() -> Rc<Options> {
    let mut opts = Options::default();

    let base_dirs = xdg::BaseDirectories::with_prefix("sway-easyfocus");
    let config_path = base_dirs
        .place_config_file("config.yaml")
        .expect("failed to create config directory");

    if let Ok(args) = Figment::new()
        .merge(Yaml::file(&config_path))
        .extract::<cli::Args>()
    {
        opts.merge(&args);
    }

    let cli_args = cli::Args::parse();
    opts.merge(&cli_args);

    Rc::new(opts)
}

fn main() {
    let _ = Lockfile::new();

    let opts = read_options();

    match swayipc::Connection::new() {
        Ok(conn) => ui::run_ui(conn, opts),
        Err(_) => eprintln!("Failed to connect to sway."),
    }
}
