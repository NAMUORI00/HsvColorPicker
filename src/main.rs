mod app;
mod capture;
mod detection;
mod settings;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 520.0])
            .with_title("HSV Control"),
        ..Default::default()
    };

    eframe::run_native(
        "HSV Color Picker",
        options,
        Box::new(|cc| Ok(Box::new(app::HsvColorPickerApp::new(cc)))),
    )
}
