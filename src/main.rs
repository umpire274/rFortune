use clap::Parser;
use rfortune::log::ConsoleLog;
use rfortune::{config, loader, utils};

mod cli;
mod commands;

use cli::{CacheAction, Cli, Commands, ConfigAction, FileAction};

fn main() {
    let cli = Cli::parse();

    println!();

    match cli.command {
        Some(Commands::Config {
            action: ConfigAction::Init,
        }) => {
            commands::run_config_init();
        }
        Some(Commands::File {
            action: FileAction::Init,
        }) => {
            commands::run_file_init();
        }
        Some(Commands::Cache {
            action: CacheAction::Clear,
        }) => {
            commands::run_cache_clear();
        }
        None => {
            // Comportamento standard: stampa una citazione casuale
            let file_path = if let Some(path) = cli.file {
                std::path::PathBuf::from(path)
            } else {
                config::get_default_path()
            };

            match loader::FortuneFile::from_file(&file_path) {
                Ok(fortune_file) => {
                    if let Err(e) = utils::print_random(&fortune_file, &file_path) {
                        ConsoleLog::ko(format!("Failed to print fortune: {e}"))
                    }
                }
                Err(e) => ConsoleLog::ko(format!("Error loading fortune file: {e}")),
            }
        }
    }
}
