use clap::Parser;
use rfortune::config::Config;
use rfortune::log::ConsoleLog;
use rfortune::utils::ensure_app_initialized;
use rfortune::{config, utils};
use std::path::Path;

mod cli;
mod commands;

use cli::{CacheAction, Cli, Commands, ConfigAction, FileAction};

fn main() {
    let cli = Cli::parse();

    println!();

    if let Err(e) = ensure_app_initialized() {
        ConsoleLog::ko(format!("Initialization error: {e}"));
        return;
    }

    // ✅ CARICHIAMO LA CONFIG UNA VOLTA QUI
    let config = config::load_config().unwrap_or_else(|| {
        ConsoleLog::warn("No configuration file found. Using defaults.");
        Config {
            default_file: Some(config::get_default_path().to_string_lossy().to_string()),
            print_title: Some(true),
            use_cache: Some(true),
            fortune_files: vec![],
        }
    });

    match cli.command {
        // ---------------- CONFIG ----------------
        Some(Commands::Config { action }) => match action {
            ConfigAction::Init => {
                commands::run_config_init();
            }
            ConfigAction::Edit { editor } => {
                commands::run_config_edit(editor);
            }
        },

        // ---------------- FILE ----------------
        Some(Commands::File { action }) => match action {
            FileAction::Init => {
                commands::run_file_init();
            }
        },

        // ---------------- CACHE ----------------
        Some(Commands::Cache { action }) => match action {
            CacheAction::Clear => {
                commands::run_cache_clear();
            }
        },

        // ---------------- DEFAULT: print random fortune ----------------
        None => {
            // 1. Risolve la PRIORITÀ delle sorgenti
            let sources = utils::resolve_fortune_sources(cli.files.clone(), &config);

            if sources.is_empty() {
                ConsoleLog::ko("No fortune sources configured or provided.");
                return;
            }

            // 2. Convertiamo in Path
            let paths: Vec<&Path> = sources.iter().map(Path::new).collect();

            // 3. Stampa citazione casuale da più file
            if let Err(e) = utils::print_random_from_files(&paths) {
                ConsoleLog::ko(format!("Failed to print fortune: {e}"));
            }
        }
    }
}
