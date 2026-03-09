use clap::Parser;
use rsimagetag::cli::{Cli, Commands, print_completions};
use rsimagetag::MyApp;
use std::path::PathBuf;

fn main() -> eframe::Result {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tag { dir } => {
            let app = match dir {
                Some(d) => MyApp::with_dir(&PathBuf::from(d)),
                None => MyApp::default(),
            };
            let options = eframe::NativeOptions {
                viewport: eframe::egui::ViewportBuilder::default()
                    .with_inner_size([800.0, 600.0]),
                ..Default::default()
            };
            eframe::run_native(
                "rsimagetag",
                options,
                Box::new(move |_cc| Ok(Box::new(app))),
            )
        }
        Commands::DbInit => {
            if let Err(e) = rsimagetag::db::init_db() {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
            Ok(())
        }
        Commands::DbDump => {
            match rsimagetag::db::dump_json() {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
            Ok(())
        }
        Commands::Complete { shell } => {
            print_completions(shell);
            Ok(())
        }
        Commands::Version => {
            println!("rsimagetag {} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
            println!("GIT_DESCRIBE: {}", env!("GIT_DESCRIBE"));
            println!("GIT_SHA: {}", env!("GIT_SHA"));
            println!("GIT_BRANCH: {}", env!("GIT_BRANCH"));
            println!("GIT_DIRTY: {}", env!("GIT_DIRTY"));
            println!("RUSTC_SEMVER: {}", env!("RUSTC_SEMVER"));
            println!("RUST_EDITION: {}", env!("RUST_EDITION"));
            println!("BUILD_TIMESTAMP: {}", env!("BUILD_TIMESTAMP"));
            Ok(())
        }
    }
}
