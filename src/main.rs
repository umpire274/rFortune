mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let paths = if args.len() > 1 {
		args[1..].to_vec()
	} else {
		vec!["/usr/local/share/games/fortune/ivespoken.dat".to_string()]
	};

	let paths: Vec<String> = paths.iter().map(|path| utils::add_dat_extension_if_needed(path.clone())).collect();

	let mut all_files = Vec::new();

	for path in paths {
		let path = Path::new(&path);
		match utils::collect_files(path) {
			Ok(mut files) => all_files.append(&mut files),
			Err(-1) => eprintln!("Errore: Il percorso '{}' non esiste o non è accessibile.", path.display()),
			Err(_) => eprintln!("Errore sconosciuto nel percorso '{}'", path.display()),
		}
	}

	if all_files.is_empty() {
		eprintln!("Nessun file trovato.");
		return Ok(());
	}

	let fortunes = utils::read_fortunes_from_files(&all_files)?;

	let unique_fortunes: Vec<String> = fortunes.into_iter().collect::<HashSet<_>>().into_iter().collect();

	let mut rng = thread_rng();
	if let Some(fortune) = unique_fortunes.choose(&mut rng) {
		println!("{}", fortune);
	} else {
		eprintln!("Nessuna frase disponibile.");
	}

	Ok(())
}
