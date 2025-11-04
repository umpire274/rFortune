use crate::log::ConsoleLog;
use dirs::data_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_file: Option<String>,
    pub print_title: Option<bool>,
    pub use_cache: Option<bool>,
}

pub(crate) fn app_dir() -> PathBuf {
    let mut base = data_dir().unwrap_or_else(|| {
        // fallback molto conservativo
        let mut p = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        p.push(".rfortune");
        p
    });
    base.push("rfortune");
    // NB: non creiamo qui la directory per non sorprendere l'utente in chiamate read-only
    base
}

pub fn get_config_path() -> PathBuf {
    let mut p = app_dir();
    p.push("rfortune.conf");
    p
}

/// Percorso di default del file fortune dell'app (rfortune.dat)
pub fn get_default_path() -> PathBuf {
    let mut p = app_dir();
    p.push("rfortune.dat");
    p
}

/// Crea un file di configurazione minimale (se non esiste)
pub fn init_config_file() -> std::io::Result<()> {
    let dir = app_dir();
    fs::create_dir_all(&dir)?;

    if let Err(e) = migrate_old_config() {
        ConsoleLog::warn(format!("Migration warning: {e}"));
    }

    let path = get_config_path();
    if path.exists() {
        ConsoleLog::info("Configuration file already exists, skipping.");
        return Ok(());
    }

    if let Err(e) = init_default_file() {
        ConsoleLog::ko(format!("Error initializing fortune file: {e}"));
    }

    let cfg = Config {
        default_file: Some(get_default_path().to_string_lossy().to_string()),
        print_title: Some(true),
        use_cache: Some(true),
    };
    let yaml = serde_yaml::to_string(&cfg).expect("Failed to serialize config");
    fs::write(path, yaml)?;

    ConsoleLog::ok("Configuration file successfully created.");
    Ok(())
}

/// Crea un file fortune di esempio (rfortune.dat) se assente
pub fn init_default_file() -> std::io::Result<()> {
    let dir = app_dir();
    fs::create_dir_all(&dir)?;

    let path = get_default_path();
    if path.exists() {
        return Ok(()); // non sovrascrivere
    }

    let sample = r#"Fortune favors the bold.
— Publius Vergilius Maro
%
Premature optimization is the root of all evil.
— Donald Knuth
%
In Rust we trust.
"#;
    fs::write(path, sample)
}

/// Carica la configurazione se presente
pub fn load_config() -> Option<Config> {
    let path = get_config_path();
    let content = fs::read_to_string(path).ok()?;
    serde_yaml::from_str(&content).ok()
}

/// Tenta di migrare una vecchia configurazione `config.yaml` a `rfortune.conf`
pub fn migrate_old_config() -> std::io::Result<()> {
    let dir = app_dir();
    let old_path = dir.join("config.yaml");
    let new_path = get_config_path();

    // Se non c'è un vecchio file o esiste già il nuovo, non fare nulla
    if !old_path.exists() || new_path.exists() {
        return Ok(());
    }

    ConsoleLog::info("Old configuration file detected — attempting migration…");

    // Legge e prova a deserializzare il vecchio file
    match fs::read_to_string(&old_path) {
        Ok(content) => match serde_yaml::from_str::<Config>(&content) {
            Ok(cfg) => {
                // Serializza e salva nel nuovo formato
                let yaml =
                    serde_yaml::to_string(&cfg).expect("Failed to serialize migrated config");
                fs::write(&new_path, yaml)?;

                // Rinomina il vecchio file come backup
                let backup = dir.join("config.yaml.bak");
                fs::rename(&old_path, &backup)?;

                ConsoleLog::ok(format!(
                    "Configuration migrated successfully → {:?}",
                    new_path
                ));
                ConsoleLog::info(format!("Backup saved as {:?}", backup));
            }
            Err(e) => {
                ConsoleLog::ko(format!("Failed to parse old config.yaml: {e}"));
            }
        },
        Err(e) => {
            ConsoleLog::ko(format!("Failed to read old config.yaml: {e}"));
        }
    }

    Ok(())
}
