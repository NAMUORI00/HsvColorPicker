mod app;
mod capture;
mod detection;
mod settings;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_title("HSV Color Picker"),
        ..Default::default()
    };

    eframe::run_native(
        "HSV Color Picker",
        options,
        Box::new(|cc| Ok(Box::new(app::HsvColorPickerApp::new(cc)))),
    )
}
