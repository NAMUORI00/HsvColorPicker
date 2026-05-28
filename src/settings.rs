use crate::detection::HsvRange;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const SETTINGS_FILE: &str = "hsv_settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsvSettings {
    pub hue_min: u8,
    pub hue_max: u8,
    pub sat_min: u8,
    pub sat_max: u8,
    pub val_min: u8,
    pub val_max: u8,
}

impl Default for HsvSettings {
    fn default() -> Self {
        Self {
            hue_min: 0,
            hue_max: 179,
            sat_min: 0,
            sat_max: 255,
            val_min: 0,
            val_max: 255,
        }
    }
}

impl HsvSettings {
    pub fn to_hsv_range(&self) -> HsvRange {
        HsvRange {
            h_lower: self.hue_min,
            h_upper: self.hue_max,
            s_lower: self.sat_min,
            s_upper: self.sat_max,
            v_lower: self.val_min,
            v_upper: self.val_max,
        }
    }

    pub fn clamp_min_max(&mut self) {
        if self.hue_min > self.hue_max {
            self.hue_min = self.hue_max;
        }
        if self.sat_min > self.sat_max {
            self.sat_min = self.sat_max;
        }
        if self.val_min > self.val_max {
            self.val_min = self.val_max;
        }
    }
}

pub fn load_settings(path: &Path) -> HsvSettings {
    if !path.exists() {
        return HsvSettings::default();
    }

    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => HsvSettings::default(),
    }
}

pub fn save_settings(path: &Path, settings: &HsvSettings) -> Result<(), String> {
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {e}"))?;
    fs::write(path, json).map_err(|e| format!("Failed to write settings: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn load_existing_hsv_settings_json() {
        let path = env::current_dir()
            .unwrap()
            .join(SETTINGS_FILE);
        if !path.exists() {
            return;
        }

        let settings = load_settings(&path);
        assert_eq!(settings.hue_min, 57);
        assert_eq!(settings.hue_max, 179);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let path = env::temp_dir().join("hsv_settings_test.json");
        let settings = HsvSettings {
            hue_min: 10,
            hue_max: 50,
            sat_min: 100,
            sat_max: 200,
            val_min: 30,
            val_max: 240,
        };

        save_settings(&path, &settings).unwrap();
        let loaded = load_settings(&path);
        assert_eq!(loaded.hue_min, 10);
        assert_eq!(loaded.hue_max, 50);
        let _ = fs::remove_file(path);
    }
}
