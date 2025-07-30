use crate::loader::FortuneFile;
use rand::seq::IndexedRandom;
use std::fs;
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
