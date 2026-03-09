pub use eframe::egui;

pub struct MyApp {
    pub name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
        }
    }
}

impl MyApp {
    pub fn greeting(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(self.greeting());
        });
    }
}
