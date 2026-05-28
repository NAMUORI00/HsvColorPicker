use crate::detection::hsv::HsvPixel;

#[derive(Clone, Copy, Debug)]
pub struct HsvRange {
    pub h_lower: u8,
    pub h_upper: u8,
    pub s_lower: u8,
    pub s_upper: u8,
    pub v_lower: u8,
    pub v_upper: u8,
}

impl Default for HsvRange {
    fn default() -> Self {
        Self {
            h_lower: 0,
            h_upper: 179,
            s_lower: 0,
            s_upper: 255,
            v_lower: 0,
            v_upper: 255,
        }
    }
}

pub fn check_color_range(h: u8, s: u8, v: u8, range: &HsvRange) -> bool {
    let h_match = if range.h_lower <= range.h_upper {
        h >= range.h_lower && h <= range.h_upper
    } else {
        h >= range.h_lower || h <= range.h_upper
    };

    let s_match = s >= range.s_lower && s <= range.s_upper;
    let v_match = v >= range.v_lower && v <= range.v_upper;

    h_match && s_match && v_match
}

pub fn create_mask(hsv: &[HsvPixel], width: u32, height: u32, range: &HsvRange) -> Vec<u8> {
    let len = (width * height) as usize;
    let mut mask = vec![0u8; len];

    for (idx, px) in hsv.iter().enumerate().take(len) {
        if check_color_range(px.h, px.s, px.v, range) {
            mask[idx] = 255;
        }
    }

    mask
}
