use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::io;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "rsimagetag")]
#[command(version = concat!(env!("CARGO_PKG_VERSION")))]
#[command(about = "Photo tagging and organization tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch the GUI to browse and tag images in the current directory
    Tag {
        /// Directory to scan for images (default: current directory)
        #[arg(short, long)]
        dir: Option<String>,
    },
    /// Initialize the tag database at ~/.config/rsimagetag/
    DbInit,
    /// Dump the entire database contents as JSON
    DbDump,
    /// Import people from a JSON file (e.g., from `rscontacts export-json --short`)
    DbImportPeople {
        /// Path to the JSON file containing [{resourceName, displayName}, ...]
        file: String,
    },
    /// Generate shell completion scripts
    Complete {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Print version information
    Version,
}

/// Parse a shell name string into a Shell enum
pub fn parse_shell(name: &str) -> Option<Shell> {
    <Shell as FromStr>::from_str(name).ok()
}

/// Generate shell completions and print to stdout
pub fn print_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "rsimagetag", &mut io::stdout());
}
