use egui;
use crate::grit;

#[derive(Default)]
pub struct App {
    grit_status: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            grit_status: grit::hello(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GBA Image Editor");

            ui.separator();

            ui.label("Status:");
            ui.label(&self.grit_status);

            ui.separator();

            ui.group(|ui| {
                ui.label("Tile Sheet Input");
                if ui.button("Load Image...") {
                    // TODO: File dialog to load tile sheet
                }
            });

            ui.group(|ui| {
                ui.label("Tilemap Options");
                // TODO: Options panels for:
                // - Bit depth selector
                // - Compression options
                // - Palette management
            });

            ui.group(|ui| {
                ui.label("Preview");
                ui.label("(Preview will render here)");
                // TODO: Image preview canvas
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Process") {
                    // TODO: Call grit processing
                }
                if ui.button("Export") {
                    // TODO: Save binary output
                }
            });
        });
    }
}
