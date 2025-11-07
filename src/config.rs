use crate::log::ConsoleLog;
use dirs::data_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_file: Option<String>,
    pub print_title: Option<bool>,
    pub use_cache: Option<bool>,
    #[serde(default)]
    pub fortune_files: Vec<String>,
}

static APP_DIR_OVERRIDE: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

/// Usata nei test per forzare una directory sandbox
pub fn set_app_dir_for_tests(dir: PathBuf) {
    let m = APP_DIR_OVERRIDE.get_or_init(|| Mutex::new(None));
    let mut guard = m.lock().unwrap();
    *guard = Some(dir);
}

/// Restituisce la directory di configurazione dell'app.
/// Durante i test, se `set_app_dir_for_tests()` è stato chiamato,
/// allora viene utilizzata la sandbox invece della directory reale.
pub fn app_dir() -> PathBuf {
    if let Some(m) = APP_DIR_OVERRIDE.get() {
        if let Some(dir) = m.lock().unwrap().clone() {
            return dir;
        }
    }

    // 1️⃣ Caso normale: dirs::data_dir() restituisce un path valido
    if let Some(mut base) = data_dir() {
        base.push("rfortune");
        return base;
    }

    // 2️⃣ Fallback: se data_dir() == None, prova con $HOME
    if let Some(home) = dirs::home_dir() {
        let mut base = home;
        #[cfg(target_os = "macos")]
        {
            base.push("Library");
            base.push("Application Support");
        }
        #[cfg(target_os = "linux")]
        {
            base.push(".local");
            base.push("share");
        }
        base.push("rfortune");
        return base;
    }

    // 3️⃣ Ultimo fallback (raro): directory corrente
    let mut p = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    p.push(".rfortune");
    p
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
        fortune_files: vec![],
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
    let content = fs::read_to_string(&path).ok()?;
    let mut cfg: Config = serde_yaml::from_str(&content).ok()?;

    // ✅ MIGRAZIONE PURA: default_file → fortune_files
    if cfg.fortune_files.is_empty()
        && let Some(df) = &cfg.default_file
    {
        cfg.fortune_files = vec![df.clone()];
        let _ = cfg.save();
    }

    Some(cfg)
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

/// Apre il file di configurazione in modifica.
/// Se viene specificato `--editor`, usa quello; altrimenti tenta di rilevare l’editor di sistema.
pub fn run_config_edit(editor_arg: Option<String>) -> std::io::Result<()> {
    let path: PathBuf = get_config_path();

    // Se non esiste, crealo
    if !path.exists() {
        ConsoleLog::warn("Configuration file not found. Creating a new one...");
        if let Err(e) = init_config_file() {
            ConsoleLog::ko(format!("Failed to create configuration file: {e}"))
        }
    }

    // 1️⃣ Priorità all'argomento CLI --editor
    let editor = if let Some(e) = editor_arg {
        e
    } else {
        // 2️⃣ Poi variabili d'ambiente VISUAL o EDITOR
        if let Ok(visual) = env::var("VISUAL") {
            visual
        } else if let Ok(editor) = env::var("EDITOR") {
            editor
        } else {
            // 3️⃣ Fallback per piattaforma
            if cfg!(target_os = "windows") {
                "notepad".to_string()
            } else {
                "nano".to_string()
            }
        }
    };

    ConsoleLog::info(format!(
        "Opening configuration file with editor: {}",
        editor
    ));

    // 4️⃣ Avvia l’editor
    let status = Command::new(&editor)
        .arg(path.to_string_lossy().to_string())
        .status();

    match status {
        Ok(s) if s.success() => ConsoleLog::ok("Configuration file closed successfully."),
        Ok(_) => ConsoleLog::warn("Editor exited with a non-zero status code."),
        Err(e) => ConsoleLog::ko(format!("Failed to launch editor '{}': {e}", editor)),
    }

    Ok(())
}

impl Config {
    /// Salva la configurazione corrente su disco (YAML).
    pub fn save(&self) -> Result<(), String> {
        let path = get_config_path();
        let parent = path.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Could not create config directory: {e}"))?;
        }

        let yaml =
            serde_yaml::to_string(&self).map_err(|e| format!("Could not serialize config: {e}"))?;

        fs::write(&path, yaml).map_err(|e| format!("Could not write config file: {e}"))
    }
}
