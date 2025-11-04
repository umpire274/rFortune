use crate::{config, utils};
use rfortune::log::ConsoleLog;

pub fn run_config_init() {
    ConsoleLog::info("Initializing configuration file...");
    if let Err(e) = config::init_config_file() {
        ConsoleLog::ko(format!("Error initializing config: {e}"));
    }
}

pub fn run_file_init() {
    ConsoleLog::info("Initializing default fortune file...");
    if let Err(e) = config::init_default_file() {
        ConsoleLog::ko(format!("Error initializing fortune file: {e}"));
    }
}

pub fn run_cache_clear() {
    ConsoleLog::info("Clearing cache directory...");
    if let Err(e) = utils::clear_cache_dir() {
        ConsoleLog::ko(format!("Error clearing cache: {e}"));
    }
}
