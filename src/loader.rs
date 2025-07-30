use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct FortuneFile {
    pub title: Option<String>,
    pub quotes: Vec<String>,
}

impl FortuneFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = fs::File::open(&path).map_err(|e| format!("Failed to open file: {e}"))?;
        let reader = BufReader::new(file);

        let mut title: Option<String> = None;
        let mut quotes: Vec<String> = Vec::new();
        let mut current_quote = String::new();
        let mut is_first_line = true;

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {e}"))?;
            let trimmed = line.trim();

            if is_first_line && trimmed.starts_with('#') {
                title = Some(trimmed.trim_start_matches('#').trim().to_string());
                is_first_line = false;
                continue;
            }

            if is_first_line && !trimmed.is_empty() {
                is_first_line = false;
            }

            if trimmed == "%" {
                if !current_quote.trim().is_empty() {
                    quotes.push(current_quote.trim().to_string());
                    current_quote.clear();
                }
            } else {
                current_quote.push_str(line.as_str());
                current_quote.push('\n');
            }
        }

        if !current_quote.trim().is_empty() {
            quotes.push(current_quote.trim().to_string());
        }

        if quotes.is_empty() {
            return Err("No quotes found in the file.".to_string());
        }

        Ok(FortuneFile { title, quotes })
    }
}
