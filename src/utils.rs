use crate::loader::FortuneFile;
use rand::seq::IndexedRandom;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn random_quote(quotes: &[String]) -> &str {
    let mut rng = rand::rng();
    quotes.choose(&mut rng).map(|s| s.as_str()).unwrap()
}

pub fn print_random(fortune_file: &FortuneFile, file_path: &Path) {
    let cache_path = get_cache_path(file_path);
    let last_used = read_last_cache(&cache_path);
    let quote = random_nonrepeating(&fortune_file.quotes, last_used.as_deref());

    if let Some(title) = &fortune_file.title {
        println!("{}\n", title);
    }
    println!("{}", quote);

    write_last_cache(&cache_path, quote);
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

pub fn get_default_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Some(home_dir) = dirs::data_dir() {
            return home_dir.join("rfortune").join("rfortunes.dat");
        }
        PathBuf::from("C:\\Users\\Public\\rfortune\\rfortunes.dat")
    } else {
        PathBuf::from("/usr/local/share/rfortune/rfortunes.dat")
    }
}

pub fn init_default_file() -> Result<(), String> {
    let path = get_default_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let mut file = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
    let sample = "%\nThe best way to get a good idea is to get a lot of ideas.\n%\nDo or do not. There is no try.\n%\nTo iterate is human, to recurse divine.\n%\n";
    file.write_all(sample.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    println!("Initialized default fortune file at: {}", path.display());
    Ok(())
}

pub fn clear_cache_dir() -> Result<(), String> {
    if let Some(mut cache_dir) = dirs::data_local_dir() {
        cache_dir.push("rfortune");
        cache_dir.push("cache");

        if cache_dir.exists() {
            fs::remove_dir_all(&cache_dir)
                .map_err(|e| format!("Failed to remove cache directory: {}", e))?;
            Ok(())
        } else {
            Ok(()) // Nessuna directory da cancellare
        }
    } else {
        Err("Unable to determine system data directory.".to_string())
    }
}
