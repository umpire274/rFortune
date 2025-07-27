mod fortune;
mod loader;

use clap::{Arg, Command};
use fortune::get_random_fortune;
use loader::load_fortunes;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_default_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Some(home_dir) = dirs::data_dir() {
            return home_dir.join("rfortune").join("rfortunes.dat");
        }
        PathBuf::from("C:\\Users\\Public\\rfortune\\rfortunes.dat")
    } else {
        PathBuf::from("/usr/local/share/rfortune/rfortunes.dat")
    }
}

fn init_default_file() -> Result<(), String> {
    let path = get_default_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|e| format!("Failed to create directory: {e}"))?;
    }

    let mut file = File::create(&path).map_err(|e| format!("Failed to create file: {e}"))?;
    let sample = "%\nThe best way to get a good idea is to get a lot of ideas.\n%\nDo or do not. There is no try.\n%\nTo iterate is human, to recurse divine.\n%\n";
    file.write_all(sample.as_bytes())
        .map_err(|e| format!("Failed to write to file: {e}"))?;

    println!("Initialized default fortune file at: {}", path.display());
    Ok(())
}

fn main() {
    let matches = Command::new("rFortune")
        .version(VERSION)
        .author("Alessandro Maestri <your@email.com>")
        .about("A Rust-based clone of the classic 'fortune' tool")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to the .dat fortune file")
                .num_args(1),
        )
        .arg(
            Arg::new("init")
                .long("init")
                .help("Initialize default fortune file and directory")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("init") {
        if let Err(e) = init_default_file() {
            eprintln!("Initialization error: {e}");
        }
        return;
    }

    let filepath = matches
        .get_one::<String>("file")
        .map(PathBuf::from)
        .unwrap_or_else(get_default_path);

    match load_fortunes(&filepath) {
        Ok(fortunes) => {
            if let Some(f) = get_random_fortune(&fortunes) {
                println!("{f}");
            } else {
                eprintln!("No fortune could be selected.");
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
