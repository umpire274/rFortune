mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::env;
use std::path::Path;

const HELP: &str = "\
Alternative implementation of the 'fortune' FreeBSD game written in Rust

USAGE:
    ivespoken [FLAGS] [file/directory]

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints the app version
";

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()> {
    let paths = handle_args()?;

    let paths: Vec<String> = paths
        .iter()
        .map(|path| utils::add_dat_extension_if_needed(path.clone()))
        .collect();

    let mut all_files = Vec::new();

    for path in paths {
        let path = Path::new(&path);
        match utils::collect_files(path) {
            Ok(mut files) => all_files.append(&mut files),
            Err(-1) => eprintln!(
                "Errore: Il percorso '{}' non esiste o non è accessibile.",
                path.display()
            ),
            Err(_) => eprintln!("Errore sconosciuto nel percorso '{}'", path.display()),
        }
    }

    if all_files.is_empty() {
        eprintln!("Nessun file trovato.");
        return Ok(());
    }

    let fortunes = utils::read_fortunes_from_files(&all_files)?;

    let unique_fortunes: Vec<String> = fortunes
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let mut rng = thread_rng();
    if let Some(fortune) = unique_fortunes.choose(&mut rng) {
        println!("{}", fortune);
    } else {
        eprintln!("Nessuna frase disponibile.");
    }

    Ok(())
}

/// Gestisce gli argomenti della riga di comando
fn handle_args() -> Result<Vec<String>, std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                println!("{}", HELP);
                std::process::exit(0);
            }
            "-v" | "--version" => {
                println!("Version: {}", VERSION);
                std::process::exit(0);
            }
            _ => Ok(args[1..].to_vec()),
        }
    } else {
        Ok(vec![
            "/usr/local/share/games/fortunes/ivespoken.ivs".to_string()
        ])
    }
}
