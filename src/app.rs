use crate::capture::ScreenCapture;
use crate::detection::{DetectResult, Detector, HsvRange, RgbImage};
use crate::settings::{HsvSettings, SETTINGS_FILE, load_settings, save_settings};
use crossbeam_channel::{Receiver, Sender, bounded};
use eframe::egui;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const QUEUE_CHECK_MS: u64 = 15;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MonitoringMode {
    Realtime,
    Static,
}

pub struct FrameUpdate {
    pub original: RgbImage,
    pub mask: Vec<u8>,
    pub bbox_frame: RgbImage,
    pub hsv_range: HsvRange,
}

pub struct SharedState {
    pub exit: AtomicBool,
    pub mode: std::sync::Mutex<MonitoringMode>,
    pub monitor_index: std::sync::Mutex<usize>,
    pub hsv_range: std::sync::Mutex<HsvRange>,
}

impl SharedState {
    fn new(range: HsvRange) -> Self {
        Self {
            exit: AtomicBool::new(false),
            mode: std::sync::Mutex::new(MonitoringMode::Realtime),
            monitor_index: std::sync::Mutex::new(0),
            hsv_range: std::sync::Mutex::new(range),
        }
    }
}

pub struct HsvColorPickerApp {
    capture: ScreenCapture,
    detector: Detector,
    settings: HsvSettings,
    settings_path: PathBuf,
    mode: MonitoringMode,
    monitor_index: usize,
    show_monitor: bool,
    static_frame: Option<RgbImage>,
    status_message: String,
    frame_rx: Receiver<FrameUpdate>,
    shared: Arc<SharedState>,
    worker_handle: Option<thread::JoinHandle<()>>,
    tex_original: Option<egui::TextureHandle>,
    tex_mask: Option<egui::TextureHandle>,
    tex_bbox: Option<egui::TextureHandle>,
    last_frame_size: (u32, u32),
}

impl HsvColorPickerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings_path = PathBuf::from(SETTINGS_FILE);
        let settings = load_settings(&settings_path);
        let hsv_range = settings.to_hsv_range();

        let capture = ScreenCapture::new().expect("Failed to initialize screen capture");
        let detector = Detector::new(hsv_range);

        let shared = Arc::new(SharedState::new(hsv_range));
        let (frame_tx, frame_rx) = bounded::<FrameUpdate>(8);

        let worker_shared = Arc::clone(&shared);
        let worker_handle = thread::spawn(move || worker_loop(worker_shared, frame_tx));

        Self {
            capture,
            detector,
            settings,
            settings_path,
            mode: MonitoringMode::Realtime,
            monitor_index: 0,
            show_monitor: false,
            static_frame: None,
            status_message: String::new(),
            frame_rx,
            shared,
            worker_handle: Some(worker_handle),
            tex_original: None,
            tex_mask: None,
            tex_bbox: None,
            last_frame_size: (0, 0),
        }
    }

    fn apply_settings_to_detector(&mut self) {
        self.settings.clamp_min_max();
        let range = self.settings.to_hsv_range();
        self.detector.set_hsv_range(range);
        *self.shared.hsv_range.lock().unwrap() = range;
    }

    fn process_static_frame(&mut self, ctx: &egui::Context) {
        let Some(frame) = self.static_frame.clone() else {
            return;
        };

        let range = self.settings.to_hsv_range();
        self.detector.set_hsv_range(range);
        let result = self.detector.detect(&frame);
        self.update_textures(ctx, &frame, &result, range);
    }

    fn update_textures(
        &mut self,
        ctx: &egui::Context,
        original: &RgbImage,
        result: &DetectResult,
        hsv_range: HsvRange,
    ) {
        let _ = hsv_range;
        let (w, h) = (original.width, original.height);
        self.last_frame_size = (w, h);

        let original_color =
            egui::ColorImage::from_rgb([w as usize, h as usize], &original.data);
        self.tex_original = Some(ctx.load_texture(
            "original",
            original_color,
            egui::TextureOptions::LINEAR,
        ));

        let mut mask_rgb = Vec::with_capacity((w * h * 3) as usize);
        for &v in &result.mask {
            mask_rgb.push(v);
            mask_rgb.push(v);
            mask_rgb.push(v);
        }
        let mask_color = egui::ColorImage::from_rgb([w as usize, h as usize], &mask_rgb);
        self.tex_mask = Some(ctx.load_texture(
            "mask",
            mask_color,
            egui::TextureOptions::LINEAR,
        ));

        let bbox_color =
            egui::ColorImage::from_rgb([w as usize, h as usize], &result.bbox_frame.data);
        self.tex_bbox = Some(ctx.load_texture(
            "bbox",
            bbox_color,
            egui::TextureOptions::LINEAR,
        ));
    }

    fn drain_latest_frame(&mut self, ctx: &egui::Context) {
        let mut latest = None;
        while let Ok(frame) = self.frame_rx.try_recv() {
            latest = Some(frame);
        }

        if let Some(update) = latest {
            let result = DetectResult {
                mask: update.mask,
                objects: vec![],
                bbox_frame: update.bbox_frame.clone(),
            };
            self.update_textures(ctx, &update.original, &result, update.hsv_range);
        }
    }

    fn hsv_range_label(&self) -> String {
        format!(
            "HSV Range: H[{}-{}], S[{}-{}], V[{}-{}]",
            self.settings.hue_min,
            self.settings.hue_max,
            self.settings.sat_min,
            self.settings.sat_max,
            self.settings.val_min,
            self.settings.val_max,
        )
    }

    fn render_slider_row(
        ui: &mut egui::Ui,
        label: &str,
        min: &mut u8,
        max: &mut u8,
        max_val: u8,
    ) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label(label);
            ui.label("Min:");
            changed |= ui
                .add(
                    egui::Slider::new(min, 0..=max_val)
                        .show_value(true)
                        .text("min"),
                )
                .changed();
            ui.label("Max:");
            changed |= ui
                .add(
                    egui::Slider::new(max, 0..=max_val)
                        .show_value(true)
                        .text("max"),
                )
                .changed();
        });
        changed
    }

    fn on_slider_changed(&mut self, ctx: &egui::Context) {
        self.apply_settings_to_detector();
        if self.mode == MonitoringMode::Static {
            self.process_static_frame(ctx);
        }
    }

    fn on_mode_changed(&mut self, ctx: &egui::Context) {
        *self.shared.mode.lock().unwrap() = self.mode;
        if self.mode == MonitoringMode::Realtime {
            self.static_frame = None;
        }
        if self.mode == MonitoringMode::Static {
            self.process_static_frame(ctx);
        }
    }

    fn on_monitor_changed(&mut self) {
        self.capture.select_monitor(self.monitor_index);
        *self.shared.monitor_index.lock().unwrap() = self.monitor_index;
    }
}

fn worker_loop(shared: Arc<SharedState>, frame_tx: Sender<FrameUpdate>) {
    let mut capture = match ScreenCapture::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Worker failed to init capture: {e}");
            return;
        }
    };

    let mut detector = Detector::default();

    while !shared.exit.load(Ordering::Relaxed) {
        let mode = *shared.mode.lock().unwrap();
        if mode != MonitoringMode::Realtime {
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        let monitor_index = *shared.monitor_index.lock().unwrap();
        capture.select_monitor(monitor_index);

        let range = *shared.hsv_range.lock().unwrap();
        detector.set_hsv_range(range);

        match capture.capture() {
            Some(frame) => {
                let result = detector.detect(&frame);
                let update = FrameUpdate {
                    original: frame,
                    mask: result.mask,
                    bbox_frame: result.bbox_frame,
                    hsv_range: range,
                };
                let _ = frame_tx.try_send(update);
            }
            None => {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

impl eframe::App for HsvColorPickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(QUEUE_CHECK_MS));

        if self.mode == MonitoringMode::Realtime {
            self.drain_latest_frame(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HSV Control");

            ui.horizontal(|ui| {
                ui.label("Monitor:");
                let monitors: Vec<_> = self
                    .capture
                    .monitor_infos()
                    .iter()
                    .map(|info| {
                        (
                            info.index,
                            format!(
                                "Monitor {}: {}x{} ({})",
                                info.index + 1,
                                info.width,
                                info.height,
                                info.name,
                            ),
                        )
                    })
                    .collect();

                let selected = monitors
                    .iter()
                    .find(|(idx, _)| *idx == self.monitor_index)
                    .map(|(_, label)| label.clone())
                    .unwrap_or_else(|| "Monitor".to_string());

                egui::ComboBox::from_id_salt("monitor_select")
                    .selected_text(selected)
                    .show_ui(ui, |ui| {
                        for (index, label) in monitors {
                            if ui
                                .selectable_value(&mut self.monitor_index, index, label)
                                .clicked()
                            {
                                self.on_monitor_changed();
                            }
                        }
                    });
            });

            ui.separator();

            ui.group(|ui| {
                ui.label("Monitoring Mode");
                ui.horizontal(|ui| {
                    if ui
                        .radio_value(&mut self.mode, MonitoringMode::Realtime, "Real-time")
                        .clicked()
                    {
                        self.on_mode_changed(ctx);
                    }
                    if ui
                        .radio_value(&mut self.mode, MonitoringMode::Static, "Static Image")
                        .clicked()
                    {
                        self.on_mode_changed(ctx);
                    }

                    let capture_enabled = self.mode == MonitoringMode::Static;
                    if ui
                        .add_enabled(capture_enabled, egui::Button::new("Capture Static Image"))
                        .clicked()
                    {
                        if let Some(frame) = self.capture.capture() {
                            self.static_frame = Some(frame);
                            self.process_static_frame(ctx);
                            self.status_message = "Static image captured.".to_string();
                        } else {
                            self.status_message = "Failed to capture static image.".to_string();
                        }
                    }
                });
            });

            ui.separator();

            ui.group(|ui| {
                ui.label("HSV Controls");
                let mut changed = false;
                changed |= Self::render_slider_row(
                    ui,
                    "Hue:",
                    &mut self.settings.hue_min,
                    &mut self.settings.hue_max,
                    179,
                );
                changed |= Self::render_slider_row(
                    ui,
                    "Saturation:",
                    &mut self.settings.sat_min,
                    &mut self.settings.sat_max,
                    255,
                );
                changed |= Self::render_slider_row(
                    ui,
                    "Value:",
                    &mut self.settings.val_min,
                    &mut self.settings.val_max,
                    255,
                );
                if changed {
                    self.on_slider_changed(ctx);
                }
            });

            ui.label(self.hsv_range_label());

            ui.horizontal(|ui| {
                if ui.button("Load Settings").clicked() {
                    self.settings = load_settings(&self.settings_path);
                    self.apply_settings_to_detector();
                    if self.mode == MonitoringMode::Static {
                        self.process_static_frame(ctx);
                    }
                    self.status_message = format!("Settings loaded from {SETTINGS_FILE}");
                }
                if ui.button("Save Settings").clicked() {
                    match save_settings(&self.settings_path, &self.settings) {
                        Ok(()) => {
                            self.status_message = format!("Settings saved to {SETTINGS_FILE}");
                        }
                        Err(e) => self.status_message = e,
                    }
                }
            });

            if ui
                .button(if self.show_monitor {
                    "Hide Monitor"
                } else {
                    "Show Monitor"
                })
                .clicked()
            {
                self.show_monitor = !self.show_monitor;
            }

            if !self.status_message.is_empty() {
                ui.label(&self.status_message);
            }

            if self.show_monitor {
                ui.separator();
                ui.label("Detection Monitor");
                ui.label(self.hsv_range_label());

                if let Some(tex) = &self.tex_original {
                    let (w, h) = self.last_frame_size;
                    let size = egui::vec2(w as f32, h as f32);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Original");
                            ui.image((tex.id(), size));
                        });
                        if let Some(mask_tex) = &self.tex_mask {
                            ui.vertical(|ui| {
                                ui.label("Mask");
                                ui.image((mask_tex.id(), size));
                            });
                        }
                        if let Some(bbox_tex) = &self.tex_bbox {
                            ui.vertical(|ui| {
                                ui.label("Bounding Box");
                                ui.image((bbox_tex.id(), size));
                            });
                        }
                    });
                } else {
                    ui.label("No frame data yet.");
                }
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.shared.exit.store(true, Ordering::Relaxed);
        if let Some(handle) = self.worker_handle.take() {
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let _ = handle.join();
                let _ = tx.send(());
            });
            let _ = rx.recv_timeout(Duration::from_secs(2));
        }
    }
}
