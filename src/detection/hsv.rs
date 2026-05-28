use crate::detection::RgbImage;

#[derive(Clone, Copy, Debug, Default)]
pub struct HsvPixel {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

pub fn rgb_to_hsv_pixel(r: u8, g: u8, b: u8) -> HsvPixel {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let maxc = r.max(g).max(b);
    let minc = r.min(g).min(b);
    let v = maxc;
    let diff = maxc - minc;

    let s = if maxc == 0.0 { 0.0 } else { diff / maxc };

    let mut h = 0.0;
    if (maxc - minc).abs() > f64::EPSILON {
        h = if maxc == r {
            let mut hue = 60.0 * (g - b) / diff;
            if g < b {
                hue += 360.0;
            }
            hue
        } else if maxc == g {
            60.0 * (b - r) / diff + 120.0
        } else {
            60.0 * (r - g) / diff + 240.0
        };
    }

    let h = (h / 2.0).round().clamp(0.0, 180.0) as u8;
    let s = (s * 255.0).round().clamp(0.0, 255.0) as u8;
    let v = (v * 255.0).round().clamp(0.0, 255.0) as u8;

    HsvPixel { h, s, v }
}

pub fn rgb_to_hsv(image: &RgbImage) -> Vec<HsvPixel> {
    let mut hsv = Vec::with_capacity((image.width * image.height) as usize);
    for chunk in image.data.chunks_exact(3) {
        hsv.push(rgb_to_hsv_pixel(chunk[0], chunk[1], chunk[2]));
    }
    hsv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_red_hsv() {
        let px = rgb_to_hsv_pixel(255, 0, 0);
        assert_eq!(px.h, 0);
        assert_eq!(px.s, 255);
        assert_eq!(px.v, 255);
    }

    #[test]
    fn parity_with_python_bgr_algorithm() {
        let cases = [(255, 0, 0), (0, 255, 0), (0, 0, 255), (128, 64, 32)];
        let expected = [(0, 255, 255), (60, 255, 255), (120, 255, 255), (10, 191, 128)];

        for (i, &(r, g, b)) in cases.iter().enumerate() {
            let px = rgb_to_hsv_pixel(r, g, b);
            assert_eq!(
                (px.h, px.s, px.v),
                expected[i],
                "mismatch for RGB({r},{g},{b})"
            );
        }
    }
}
