use std::fmt;

/// Tipologie di messaggio per la console
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogKind {
    Info,
    Ok,
    Ko,
    Warning,
}

/// Logger per messaggi a console (no file)
pub struct ConsoleLog;

impl ConsoleLog {
    /// Stampa un messaggio con colore e simbolo in base al tipo
    pub fn log(kind: LogKind, msg: impl fmt::Display) {
        match kind {
            LogKind::Info => {
                // ℹ = U+2139
                println!("\x1b[36m\u{2139}  {}\x1b[0m", msg);
            }
            LogKind::Ok => {
                // ✅ = U+2705
                println!("\x1b[32m\u{2705} {}\x1b[0m", msg);
            }
            LogKind::Ko => {
                // ❌ = U+274C
                eprintln!("\x1b[31m\u{274C} {}\x1b[0m", msg);
            }
            LogKind::Warning => {
                // ⚠️ = U+26A0 U+FE0F
                println!("\x1b[33m\u{26A0}\u{FE0F} {}\x1b[0m", msg);
            }
        }
    }

    pub fn info(msg: impl fmt::Display) {
        Self::log(LogKind::Info, msg);
    }

    pub fn ok(msg: impl fmt::Display) {
        Self::log(LogKind::Ok, msg);
    }

    pub fn ko(msg: impl fmt::Display) {
        Self::log(LogKind::Ko, msg);
    }

    pub fn warn(msg: impl fmt::Display) {
        Self::log(LogKind::Warning, msg);
    }
}
