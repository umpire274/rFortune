use crate::config;
use crate::config::Config;
use crate::loader::FortuneFile;
use crate::log::ConsoleLog;
use anyhow::{Context, Result};
use fs2::FileExt;
use rand::seq::IndexedRandom;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

/// Estrae una citazione casuale dalla lista
pub fn random_quote(quotes: &[String]) -> &str {
    let mut rng = rand::rng();
    quotes.choose(&mut rng).map(|s| s.as_str()).unwrap()
}

/// Stampa una citazione casuale dal file fortune
pub fn print_random_from_files(paths: &[&Path]) -> Result<(), String> {
    let mut all_quotes: Vec<String> = Vec::new();
    let mut origin_by_index: Vec<PathBuf> = Vec::new();

    // 1) Carichiamo tutte le citazioni e tracciamo il file di origine
    for path in paths {
        match FortuneFile::from_file(path) {
            Ok(f) => {
                for q in &f.quotes {
                    all_quotes.push(q.clone());
                    origin_by_index.push(path.to_path_buf());
                }
            }
            Err(e) => {
                ConsoleLog::warn(format!("Could not load file {}: {e}", path.display()));
            }
        }
    }

    if all_quotes.is_empty() {
        ConsoleLog::ko("No quotes found in any of the fortune files.");
        return Err("No quotes found.".into());
    }

    // 2) Proviamo a evitare ripetizioni dal *medesimo* file
    // Carichiamo ultima citazione SOLO se deriva da uno dei file in uso
    let mut last_quote = None;

    for p in paths {
        if let Ok(q) = load_last_cache(p) {
            last_quote = Some(q);
            break; // basta la prima cache utile
        }
    }

    let quote = if let Some(last) = last_quote {
        // Rimuoviamo tutte le citazioni identiche all’ultima
        let filtered: Vec<&String> = all_quotes.iter().filter(|q| *q != &last).collect();

        if filtered.is_empty() {
            // Se tutte erano uguali (caso rarissimo), scegliamo pure
            all_quotes.choose(&mut rand::rng()).unwrap().clone()
        } else {
            filtered.choose(&mut rand::rng()).unwrap().to_string()
        }
    } else {
        // Nessuna citazione precedente → scelta libera
        all_quotes.choose(&mut rand::rng()).unwrap().clone()
    };

    // 3) Identifichiamo il file da cui la citazione proviene
    let idx = all_quotes
        .iter()
        .position(|q| q == &quote)
        .expect("internal mismatch");
    let origin = &origin_by_index[idx];

    // 4) Stampa effettiva
    println!("{quote}");

    // 5) Salviamo la cache SOLO per il file di origine
    if let Err(e) = save_last_cache(origin.as_path(), &quote) {
        ConsoleLog::warn(format!("Could not update cache: {e}"));
    }

    Ok(())
}

/// Percorso del file cache per un determinato fortune file
pub fn get_cache_path(dat_path: &Path) -> PathBuf {
    let mut base = config::app_dir();
    base.push("cache");

    // ✅ garantisce che la directory esista sempre
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
pub fn random_nonrepeating(quotes: &[String], last: Option<String>) -> &str {
    let mut rng = rand::rng();
    let filtered: Vec<&String> = quotes
        .iter()
        .filter(|q| Some(q.as_str()) != last.as_deref())
        .collect();

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

/// Ensure that the parent directory for the given cache store path exists.
/// Returns an error if creation fails.
fn ensure_cache_dir(store: &Path) -> Result<()> {
    if let Some(parent) = store.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create cache dir: {}", parent.display()))?;
    }
    Ok(())
}

/// Open the given `store` file and acquire a lock.
/// If `exclusive` is true, open for read+write+create and acquire an exclusive lock.
/// Otherwise, open read-only and acquire a shared lock.
fn open_and_lock(store: &Path, exclusive: bool) -> Result<File> {
    let file = if exclusive {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(store)
            .with_context(|| format!("open cache file: {}", store.display()))?
    } else {
        OpenOptions::new()
            .read(true)
            .open(store)
            .with_context(|| format!("open cache file (read): {}", store.display()))?
    };

    if exclusive {
        file.lock_exclusive()
            .with_context(|| format!("lock cache (exclusive): {}", store.display()))?;
    } else {
        file.lock_shared()
            .with_context(|| format!("lock cache (shared): {}", store.display()))?;
    }

    Ok(file)
}

/// Salva l’ultima citazione usata in un file di cache (scrittura atomica + locking)
pub fn save_last_cache(path: &Path, quote: &str) -> Result<()> {
    let store = cache_store_path();
    ensure_cache_dir(&store)?;

    // Apri (o crea) il file store e acquisisci lock esclusivo
    let mut file = open_and_lock(&store, true)?;

    // Leggi contenuto esistente dal file handle per rispettare il lock
    let mut existing = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut existing).ok();

    let mut map: HashMap<String, String> = if existing.is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&existing).unwrap_or_default()
    };

    map.insert(canonical_key(path), quote.to_string());

    let json = serde_json::to_string_pretty(&map).with_context(|| "serialize cache")?;

    // Scrittura atomica: crea file temporaneo nella stessa directory e rinomina
    let tmp_name = format!(
        "{}.tmp.{}",
        store
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "last_quotes.json".into()),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    );

    let tmp_path = store.with_file_name(tmp_name);

    fs::write(&tmp_path, &json)
        .with_context(|| format!("write temp cache: {}", tmp_path.display()))?;

    // Release lock and close the original store file before attempting rename/remove (important on Windows)
    file.unlock().ok();
    drop(file);

    // Try rename; on Windows older semantics may prevent overwriting, try remove+rename
    match fs::rename(&tmp_path, &store) {
        Ok(_) => {}
        Err(e) => {
            if cfg!(windows) {
                // attempt best-effort replace
                if store.exists() {
                    fs::remove_file(&store)
                        .with_context(|| format!("remove existing store: {}", store.display()))?;
                    fs::rename(&tmp_path, &store).with_context(|| {
                        format!(
                            "rename cache after remove: {} -> {}",
                            tmp_path.display(),
                            store.display()
                        )
                    })?;
                } else {
                    return Err(e).with_context(|| "rename cache failed and store did not exist");
                }
            } else {
                return Err(e).with_context(|| "rename cache failed");
            }
        }
    }

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

pub fn get_fortune_sources(cli_files: Option<Vec<String>>, config: &Config) -> Vec<String> {
    if let Some(files) = cli_files
        && !files.is_empty()
    {
        return files;
    }

    if !config.fortune_files.is_empty() {
        return config.fortune_files.clone();
    }

    if let Some(df) = &config.default_file {
        return vec![df.clone()];
    }

    // fallback hard-coded (ultima ratio)
    vec!["/usr/local/share/rfortune/fortunes".into()]
}

pub fn resolve_fortune_sources(cli_files: Option<Vec<String>>, config: &Config) -> Vec<String> {
    if let Some(files) = cli_files
        && !files.is_empty()
    {
        return files;
    }

    if !config.fortune_files.is_empty() {
        return config.fortune_files.clone();
    }

    if let Some(default) = &config.default_file {
        return vec![default.clone()];
    }

    vec![]
}

/// Percorso del file JSON di cache: ~/.local/share/rfortune/cache/last_quotes.json
fn cache_store_path() -> PathBuf {
    let mut p = config::app_dir();
    p.push("cache");
    let _ = fs::create_dir_all(&p);
    p.push("last_quotes.json");
    p
}

fn canonical_key(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .to_string()
}

/// Carica l'ULTIMA citazione mostrata per il file `path`.
/// Ritorna Ok(quote) se presente, Err(...) se assente o in caso di problema non critico.
pub fn load_last_cache(path: &Path) -> Result<String> {
    let store = cache_store_path();

    // garantisci che la directory cache esista
    if let Some(parent) = store.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create cache directory: {}", parent.display()))?;
    }

    // se il file non esiste ancora → ritorna "nessuna cache" ma senza errore fatale
    if !store.exists() {
        return Err(anyhow::anyhow!("no cache"));
    }

    // Apri file in sola lettura e acquisisci lock condiviso
    let mut file = open_and_lock(&store, false)?;

    let mut data = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut data).ok();

    let map: HashMap<String, String> = serde_json::from_str(&data).unwrap_or_default();

    let result = map
        .get(&canonical_key(path))
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no cache"));

    // Rilascia il lock (ignore unlock error)
    let _ = file.unlock();

    result
}
