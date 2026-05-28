use crate::detection::RgbImage;
use xcap::Monitor;

const CAPTURE_SIZE: u32 = 320;

#[derive(Clone, Debug)]
pub struct MonitorInfo {
    pub index: usize,
    pub width: u32,
    pub height: u32,
    pub name: String,
}

pub struct ScreenCapture {
    monitors: Vec<Monitor>,
    monitor_infos: Vec<MonitorInfo>,
    selected_index: usize,
    capture_size: u32,
}

impl ScreenCapture {
    pub fn new() -> Result<Self, String> {
        let monitors = Monitor::all().map_err(|e| format!("Failed to list monitors: {e}"))?;
        if monitors.is_empty() {
            return Err("No monitors found".to_string());
        }

        let monitor_infos = monitors
            .iter()
            .enumerate()
            .map(|(index, m)| MonitorInfo {
                index,
                width: m.width(),
                height: m.height(),
                name: m.name().to_string(),
            })
            .collect();

        Ok(Self {
            monitors,
            monitor_infos,
            selected_index: 0,
            capture_size: CAPTURE_SIZE,
        })
    }

    pub fn monitor_infos(&self) -> &[MonitorInfo] {
        &self.monitor_infos
    }

    pub fn select_monitor(&mut self, index: usize) -> bool {
        if index < self.monitors.len() {
            self.selected_index = index;
            true
        } else {
            false
        }
    }

    pub fn capture(&self) -> Option<RgbImage> {
        let monitor = self.monitors.get(self.selected_index)?;
        let rgba = monitor.capture_image().ok()?;
        let full_w = rgba.width();
        let full_h = rgba.height();

        if full_w == 0 || full_h == 0 {
            return None;
        }

        let size = self.capture_size.min(full_w).min(full_h);
        let left = (full_w - size) / 2;
        let top = (full_h - size) / 2;

        let mut data = Vec::with_capacity((size * size * 3) as usize);
        for y in top..top + size {
            for x in left..left + size {
                let pixel = rgba.get_pixel(x, y);
                data.push(pixel[0]);
                data.push(pixel[1]);
                data.push(pixel[2]);
            }
        }

        Some(RgbImage::new(size, size, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_monitors_and_capture_center() {
        let capture = ScreenCapture::new().expect("monitors should be available");
        assert!(!capture.monitor_infos().is_empty());

        let frame = capture.capture().expect("capture should succeed");
        assert_eq!(frame.width, 320);
        assert_eq!(frame.height, 320);
        assert_eq!(frame.data.len(), 320 * 320 * 3);
    }
}
