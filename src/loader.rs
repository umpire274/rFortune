// src/loader.rs
use std::fs::read_to_string;
use std::path::Path;

pub fn load_fortunes<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, String> {
    let content =
        read_to_string(filename.as_ref()).map_err(|e| format!("Failed to read file: {e}"))?;

    let fortunes: Vec<String> = content
        .split('%')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    if fortunes.is_empty() {
        Err("No fortunes found in file.".to_string())
    } else {
        Ok(fortunes)
    }
}
