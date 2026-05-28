mod contour;
mod hsv;
mod mask;
mod morph;

pub use contour::{DetectedObject, draw_objects, find_contours};
pub use hsv::rgb_to_hsv;
pub use mask::{HsvRange, create_mask};
pub use morph::dilate;

#[derive(Clone, Debug)]
pub struct RgbImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl RgbImage {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DetectResult {
    pub mask: Vec<u8>,
    pub objects: Vec<DetectedObject>,
    pub bbox_frame: RgbImage,
}

pub struct Detector {
    range: HsvRange,
}

impl Default for Detector {
    fn default() -> Self {
        Self {
            range: HsvRange::default(),
        }
    }
}

impl Detector {
    pub fn new(range: HsvRange) -> Self {
        Self { range }
    }

    pub fn set_hsv_range(&mut self, range: HsvRange) {
        self.range = range;
    }

    pub fn detect(&self, frame: &RgbImage) -> DetectResult {
        let hsv = rgb_to_hsv(frame);
        let mask = create_mask(&hsv, frame.width, frame.height, &self.range);
        let dilated = dilate(&mask, frame.width, frame.height, 3, 2);
        let contours = find_contours(&dilated, frame.width, frame.height, 20.0);

        let mut objects = Vec::new();
        for (contour, area) in contours {
            if area > 20.0 {
                let (x, y, w, h) = contour::get_bounding_rect(&contour);
                objects.push(DetectedObject {
                    x,
                    y,
                    width: w,
                    height: h,
                    area,
                });
            }
        }

        let bbox_frame = draw_objects(frame, &objects, (0, 255, 0), 2);

        DetectResult {
            mask: dilated,
            objects,
            bbox_frame,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_solid_color_block() {
        let mut data = vec![0u8; 320 * 320 * 3];
        for y in 100..200 {
            for x in 100..200 {
                let idx = ((y * 320 + x) * 3) as usize;
                data[idx] = 255;
                data[idx + 1] = 0;
                data[idx + 2] = 0;
            }
        }

        let frame = RgbImage::new(320, 320, data);
        let mut detector = Detector::default();
        detector.set_hsv_range(HsvRange {
            h_lower: 0,
            h_upper: 10,
            s_lower: 100,
            s_upper: 255,
            v_lower: 100,
            v_upper: 255,
        });

        let result = detector.detect(&frame);
        assert!(!result.objects.is_empty());
    }
}
