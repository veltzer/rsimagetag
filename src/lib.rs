pub mod cli;
pub mod db;
pub mod icon;

pub use eframe::egui;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
];

pub fn scan_images(dir: &Path) -> Vec<PathBuf> {
    let mut images: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    images.sort();
    images
}

fn load_image_as_texture(
    ctx: &egui::Context,
    path: &Path,
) -> Option<egui::TextureHandle> {
    let img = image::open(path).ok()?.into_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(
        path.to_string_lossy(),
        color_image,
        egui::TextureOptions::LINEAR,
    ))
}

pub struct MyApp {
    pub images: Vec<PathBuf>,
    pub current_index: usize,
    pub texture: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or_default();
        let images = scan_images(&cwd);
        Self {
            images,
            current_index: 0,
            texture: None,
        }
    }
}

impl MyApp {
    pub fn with_dir(dir: &Path) -> Self {
        let images = scan_images(dir);
        Self {
            images,
            current_index: 0,
            texture: None,
        }
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }

    pub fn current_path(&self) -> Option<&Path> {
        self.images.get(self.current_index).map(|p| p.as_path())
    }

    pub fn go_next(&mut self) {
        if !self.images.is_empty() {
            self.current_index = (self.current_index + 1) % self.images.len();
            self.texture = None;
        }
    }

    pub fn go_prev(&mut self) {
        if !self.images.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.images.len() - 1
            } else {
                self.current_index - 1
            };
            self.texture = None;
        }
    }

    fn load_current_texture(&mut self, ctx: &egui::Context) {
        if self.texture.is_none() {
            if let Some(path) = self.images.get(self.current_index) {
                self.texture = load_image_as_texture(ctx, path);
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_current_texture(ctx);

        // Handle keyboard navigation
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::N) {
                self.go_next();
                self.load_current_texture(ctx);
            }
            if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::P) {
                self.go_prev();
                self.load_current_texture(ctx);
            }
        });

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("<< Prev").clicked() {
                    self.go_prev();
                    self.load_current_texture(ctx);
                }
                if ui.button("Next >>").clicked() {
                    self.go_next();
                    self.load_current_texture(ctx);
                }
                ui.separator();
                if self.images.is_empty() {
                    ui.label("No images found in current directory.");
                } else {
                    ui.label(format!(
                        "{} / {}  —  {}",
                        self.current_index + 1,
                        self.images.len(),
                        self.images[self.current_index]
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy(),
                    ));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let available = ui.available_size();
                let img_size = texture.size_vec2();
                let scale = (available.x / img_size.x).min(available.y / img_size.y).min(1.0);
                let display_size = img_size * scale;
                ui.centered_and_justified(|ui| {
                    ui.image(egui::load::SizedTexture::new(texture.id(), display_size));
                });
            } else if self.images.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.heading("No images found.\nRun rsimagetag from a directory containing images.");
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.heading("Loading...");
                });
            }
        });
    }
}
