use clap::{Arg, Command};
use std::env;
use std::path::PathBuf;

use rfortune::loader::FortuneFile;
use rfortune::utils::{get_default_path, init_default_file, print_random};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        .arg(
            Arg::new("clear-cache")
                .long("clear-cache")
                .help("Delete all cached quote history")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if *matches.get_one::<bool>("clear-cache").unwrap_or(&false) {
        match rfortune::utils::clear_cache_dir() {
            Ok(_) => println!("Cache cleared successfully."),
            Err(e) => eprintln!("Error clearing cache: {e}"),
        }
        return;
    }

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

    match FortuneFile::from_file(&filepath) {
        Ok(fortune_file) => print_random(&fortune_file, &filepath),
        Err(err) => eprintln!("Error: {err}"),
    }
}
