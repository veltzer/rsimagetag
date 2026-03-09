use rsimagetag::MyApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "rsimagetag",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
