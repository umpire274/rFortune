use std::fs;
use std::io;
use std::path::Path;

// Aggiunge l'estensione ".dat" al percorso se necessario
pub fn add_dat_extension_if_needed(path: String) -> String {
	let path_with_extension = Path::new(&path).with_extension("ivs");

	if path_with_extension.is_file() && Path::new(&path).extension().is_none() {
		return format!("{}.ivs", path);
	}

	path
}

// Raccoglie i file da un percorso specificato: file singolo o directory (ricorsivamente)
pub fn collect_files(path: &Path) -> Result<Vec<String>, i32> {
	if path.is_file() {
		if let Some(ext) = path.extension() {
			if ext == "ivs" {
				return Ok(vec![path.to_string_lossy().to_string()]);
			} else {
				eprintln!("Il file '{}' non ha estensione '.ivs'.", path.display());
				return Err(-1);
			}
		}
		eprintln!("Il file '{}' non ha estensione '.ivs'.", path.display());
		return Err(-1);
	} else if path.is_dir() {
		let mut files = Vec::new();
		if let Err(e) = explore_directory(path, &mut files) {
			eprintln!("Errore nell'esplorare la directory '{}': {}", path.display(), e);
		}
		Ok(files)
	} else {
		Err(-1)
	}
}

// Esplora ricorsivamente una directory per raccogliere file con l'estensione ".dat"
fn explore_directory(dir: &Path, files: &mut Vec<String>) -> io::Result<()> {
	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		let entry_path = entry.path();

		if entry_path.is_dir() {
			explore_directory(&entry_path, files)?;
		} else if let Some(ext) = entry_path.extension() {
			if ext == "ivs" {
				files.push(entry_path.to_string_lossy().to_string());
			}
		}
	}
	Ok(())
}

// Legge e combina le fortune da più file, separandole usando il carattere "%"
pub fn read_fortunes_from_files(filenames: &[String]) -> io::Result<Vec<String>> {
	let mut all_fortunes = Vec::new();

	for filename in filenames {
		match fs::read_to_string(filename) {
			Ok(content) => {
				let fortunes: Vec<String> = content
					.split('%')
					.map(|s| s.trim().to_string())
					.filter(|s| !s.is_empty())
					.collect();
				all_fortunes.extend(fortunes);
			}
			Err(e) => {
				eprintln!("Errore leggendo il file '{}': {}", filename, e);
			}
		}
	}

	Ok(all_fortunes)
}
