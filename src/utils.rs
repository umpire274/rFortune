use crate::config;
use crate::loader::FortuneFile;
use crate::log::ConsoleLog;
use rand::seq::IndexedRandom;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Estrae una citazione casuale dalla lista
pub fn random_quote(quotes: &[String]) -> &str {
    let mut rng = rand::rng();
    quotes.choose(&mut rng).map(|s| s.as_str()).unwrap()
}

/// Stampa una citazione casuale dal file fortune
pub fn print_random(fortune_file: &FortuneFile, file_path: &Path) -> Result<(), String> {
    if fortune_file.quotes.is_empty() {
        ConsoleLog::ko("No quotes found in the fortune file.");
        return Err("No quotes found in the file.".to_string());
    }

    // Se possibile evita di ripetere l’ultima citazione
    let quote = random_nonrepeating(&fortune_file.quotes, None);

    if let Some(title) = &fortune_file.title {
        println!("({title})");
    }

    // Stampa il contenuto vero e proprio
    println!("{quote}");

    // Aggiornamento cache
    if let Err(e) = save_last_cache(file_path, quote) {
        ConsoleLog::warn(format!("Could not update cache: {e}"));
    }

    Ok(())
}

/// Percorso del file cache per un determinato fortune file
pub fn get_cache_path(dat_path: &Path) -> PathBuf {
    let mut base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    base.push("rfortune");
    base.push("cache");
    if let Err(e) = fs::create_dir_all(&base) {
        ConsoleLog::warn(format!("Unable to create cache directory: {e}"));
    }

    let name = dat_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        + ".cache";

    base.push(name);
    base
}

/// Legge l’ultima citazione salvata in cache (se esiste)
pub fn read_last_cache(path: &Path) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(content) => {
            ConsoleLog::info(format!("Loaded cached quote from '{}'.", path.display()));
            Some(content)
        }
        Err(_) => None,
    }
}

/// Scrive l’ultima citazione nella cache
pub fn write_last_cache(path: &Path, quote: &str) {
    if let Err(e) = fs::write(path, quote) {
        ConsoleLog::warn(format!(
            "Failed to write cache file '{}': {e}",
            path.display()
        ));
    }
}

/// Restituisce una citazione casuale diversa dalla precedente (se possibile)
pub fn random_nonrepeating<'a>(quotes: &'a [String], last: Option<&str>) -> &'a str {
    let mut rng = rand::rng();
    let filtered: Vec<&String> = quotes.iter().filter(|q| Some(q.as_str()) != last).collect();

    if filtered.is_empty() {
        quotes.choose(&mut rng).unwrap()
    } else {
        filtered.choose(&mut rng).unwrap()
    }
}

/// Directory base della cache
fn get_cache_dir() -> PathBuf {
    let mut p = config::app_dir();
    p.push("rfortune");
    p.push("cache");
    p
}

/// Svuota completamente la cache
pub fn clear_cache_dir() -> io::Result<()> {
    let dir = get_cache_dir();
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
        ConsoleLog::ok(format!("Cache directory cleared: {}", dir.display()));
    } else {
        ConsoleLog::info("Cache directory is already empty.");
    }
    Ok(())
}

/// Salva l’ultima citazione usata in un file di cache
pub fn save_last_cache(file_path: &Path, quote: &str) -> io::Result<()> {
    let cache_path = get_cache_path(file_path);
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?; // assicura che la cartella esista
    }

    let mut f = fs::File::create(&cache_path)?;
    f.write_all(quote.as_bytes())?;

    Ok(())
}

pub fn ensure_app_initialized() -> io::Result<()> {
    let dir = config::app_dir();

    if dir.exists() {
        return Ok(()); // tutto a posto
    }

    // Se il processo è interattivo (TTY), chiedi conferma
    if atty::is(atty::Stream::Stdin) {
        print!("Configuration directory not found. Initialize rFortune now? [Y/n]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let answer = input.trim().to_lowercase();

        if answer == "n" || answer == "no" {
            ConsoleLog::warn("Initialization aborted by user.");
            std::process::exit(0);
        }
    } else {
        ConsoleLog::info("Configuration directory missing — initializing automatically.");
    }

    // Esegui l’inizializzazione completa
    ConsoleLog::info("Initializing rFortune environment...");
    config::init_config_file()?;
    ConsoleLog::ok("rFortune initialized successfully.");
    Ok(())
}
