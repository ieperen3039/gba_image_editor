mod grit;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "GBA Image Editor",
        options,
        Box::new(|_cc| Ok(Box::new(ui::App::default()))),
    )
}
