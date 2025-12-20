use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use clap::Parser;

mod cli;
mod config;
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

fn main() {
    let args = cli::Args::parse();
    let _ = dbg!(args);

    let lock_path = make_lockfile().expect("Lockfile exists! (Is another instance running?)");

    std::fs::remove_file(&lock_path).expect(&format!(
        "Failed to remove lockfile at {}.",
        &lock_path.display()
    ));
}
