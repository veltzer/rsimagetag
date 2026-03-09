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
