use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "rfortune",
    version,
    about = "Print random quotes from fortune files",
    long_about = "rfortune is a Rust implementation of the classic UNIX 'fortune' program.\n\n\
By default, running `rfortune` prints a random quotation from the default file \
(rfortune.dat) stored in your user data directory. Quotes inside a fortune file \
must be separated by a line containing only the '%' character.\n\n\
You can specify a custom fortune file with `--file`, or manage configuration, \
fortune files, and cache using subcommands:\n\n  \
• `config init`   Create a configuration file with default options.\n  \
• `file init`     Create a sample default fortune file (rfortune.dat).\n  \
• `cache clear`   Remove all cached last-used fortunes.\n\n\
This makes it easy to test, customize and extend your fortune collections \
while preserving the spirit of the original UNIX command.",
    after_help = "EXAMPLES:\n  rfortune\n      Print a random fortune from the default file (rfortune.dat).\n\n  rfortune --file ~/fortunes/misc\n      Print a random fortune from the file ~/fortunes/misc.\n\n  rfortune config init\n      Create a default configuration file in the user data directory.\n\n  rfortune file init\n      Create a sample fortune file (rfortune.dat) in the user data directory.\n\n  rfortune cache clear\n      Remove all cached last-used fortunes."
)]
pub struct Cli {
    /// Fortune file to use instead of the default (rfortune.dat)
    #[arg(short, long)]
    pub file: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Manage fortune files
    File {
        #[command(subcommand)]
        action: FileAction,
    },
    /// Manage cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Initialize the configuration file
    Init,
}

#[derive(Subcommand, Debug)]
pub enum FileAction {
    /// Create a sample default fortune file (rfortune.dat)
    Init,
}

#[derive(Subcommand, Debug)]
pub enum CacheAction {
    /// Clear the cache directory
    Clear,
}
