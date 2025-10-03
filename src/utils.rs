use crate::config;
use crate::loader::FortuneFile;
use rand::seq::IndexedRandom;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn random_quote(quotes: &[String]) -> &str {
    let mut rng = rand::rng();
    quotes.choose(&mut rng).map(|s| s.as_str()).unwrap()
}

pub fn print_random(fortune_file: &FortuneFile, file_path: &Path) -> Result<(), String> {
    // esempio: recupera la citazione casuale
    let quote = if fortune_file.quotes.is_empty() {
        return Err("No quotes found in the file.".to_string());
    } else {
        random_nonrepeating(&fortune_file.quotes, None)
    };

    // eventuale stampa titolo se presente
    if let Some(title) = &fortune_file.title {
        println!("({title})");
    }

    // stampa il contenuto
    println!("{}", quote);

    // qui salvi anche in cache se necessario
    if let Err(e) = save_last_cache(file_path, quote) {
        eprintln!("Warning: could not update cache: {e}");
    }

    Ok(())
}

pub fn get_cache_path(dat_path: &Path) -> PathBuf {
    let mut base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    base.push("rfortune");
    base.push("cache");
    fs::create_dir_all(&base).ok();

    let name = dat_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        + ".cache";

    base.push(name);
    base
}

pub fn read_last_cache(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

pub fn write_last_cache(path: &Path, quote: &str) {
    let _ = fs::write(path, quote);
}

pub fn random_nonrepeating<'a>(quotes: &'a [String], last: Option<&str>) -> &'a str {
    let mut rng = rand::rng();
    let filtered: Vec<&String> = quotes.iter().filter(|q| Some(q.as_str()) != last).collect();

    if filtered.is_empty() {
        quotes.choose(&mut rng).unwrap()
    } else {
        filtered.choose(&mut rng).unwrap()
    }
}

/// Directory della cache: .../rfortune/cache
fn get_cache_dir() -> PathBuf {
    let mut p = config::app_dir();
    p.push("rfortune");
    p.push("cache");
    p
}

/// Svuota completamente la cache
pub fn clear_cache_dir() -> std::io::Result<()> {
    let dir = get_cache_dir();
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    Ok(())
}

/// Salva l’ultima citazione usata in un file di cache
pub fn save_last_cache(file_path: &Path, quote: &str) -> std::io::Result<()> {
    let cache_path = get_cache_path(file_path);
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?; // assicura che la cartella esista
    }
    let mut f = fs::File::create(cache_path)?;
    f.write_all(quote.as_bytes())?;
    Ok(())
}
