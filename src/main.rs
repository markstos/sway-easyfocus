use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use clap::Parser;
use figment::Figment;
use figment::providers::{Format, Yaml};

use crate::options::Options;

mod cli;
mod options;
mod util;

fn make_lockfile() -> io::Result<PathBuf> {
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
    File::create_new(&lock_path).map(move |_| lock_path)
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
    let opts = read_options();
    let _ = dbg!(opts);

    let lock_path = make_lockfile().expect("Lockfile exists! (Is another instance running?)");

    std::fs::remove_file(&lock_path).expect(&format!(
        "Failed to remove lockfile at {}.",
        &lock_path.display()
    ));
}
