use crate::{config, utils};

pub fn run_config_init() {
    if let Err(e) = config::init_config_file() {
        eprintln!("Error initializing config: {e}");
    }
}

pub fn run_file_init() {
    if let Err(e) = config::init_default_file() {
        eprintln!("Error initializing fortune file: {e}");
    }
}

pub fn run_cache_clear() {
    if let Err(e) = utils::clear_cache_dir() {
        eprintln!("Error clearing cache: {e}");
    }
}
