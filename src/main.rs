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
        Commands::Complete { shell } => {
            print_completions(shell);
            Ok(())
        }
        Commands::Version => {
            println!("rsimagetag {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}
