use crate::detection::RgbImage;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

#[derive(Clone, Debug)]
pub struct DetectedObject {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub area: f64,
}

pub fn calculate_contour_area(contour: &[(i32, i32)]) -> f64 {
    if contour.len() < 3 {
        return 0.0;
    }

    let mut sum = 0.0;
    let n = contour.len();
    for i in 0..n {
        let (x0, y0) = contour[i];
        let (x1, y1) = contour[(i + 1) % n];
        sum += (x0 as f64) * (y1 as f64) - (x1 as f64) * (y0 as f64);
    }

    0.5 * sum.abs()
}

pub fn get_bounding_rect(contour: &[(i32, i32)]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for &(x, y) in contour {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    (
        min_x,
        min_y,
        max_x - min_x + 1,
        max_y - min_y + 1,
    )
}

fn trace_contour(
    mask: &[u8],
    width: i32,
    height: i32,
    visited: &mut [bool],
    start_y: i32,
    start_x: i32,
) -> Vec<(i32, i32)> {
    let mut contour = Vec::new();
    let mut stack = vec![(start_y, start_x)];

    while let Some((y, x)) = stack.pop() {
        let idx = (y * width + x) as usize;
        if visited[idx] {
            continue;
        }
        visited[idx] = true;

        let mut is_edge = false;
        for &(dy, dx) in &DIRECTIONS {
            let ny = y + dy;
            let nx = x + dx;
            if ny >= 0 && ny < height && nx >= 0 && nx < width {
                if mask[(ny * width + nx) as usize] == 0 {
                    is_edge = true;
                    break;
                }
            } else {
                is_edge = true;
                break;
            }
        }

        if is_edge {
            contour.push((x, y));
            for &(dy, dx) in &DIRECTIONS {
                let ny = y + dy;
                let nx = x + dx;
                if ny >= 0
                    && ny < height
                    && nx >= 0
                    && nx < width
                    && mask[(ny * width + nx) as usize] > 0
                    && !visited[(ny * width + nx) as usize]
                {
                    stack.push((ny, nx));
                }
            }
        }
    }

    contour
}

pub fn find_contours(mask: &[u8], width: u32, height: u32, min_area: f64) -> Vec<(Vec<(i32, i32)>, f64)> {
    let w = width as i32;
    let h = height as i32;
    let len = (width * height) as usize;
    let mut visited = vec![false; len];
    let mut contours = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            if mask[idx] > 0 && !visited[idx] {
                let contour = trace_contour(mask, w, h, &mut visited, y, x);
                if !contour.is_empty() {
                    let area = calculate_contour_area(&contour);
                    if area >= min_area {
                        contours.push((contour, area));
                    }
                }
            }
        }
    }

    contours
}

pub fn draw_objects(
    frame: &RgbImage,
    objects: &[DetectedObject],
    color: (u8, u8, u8),
    thickness: i32,
) -> RgbImage {
    let mut output = frame.clone();
    let w = frame.width as i32;
    let h = frame.height as i32;

    for obj in objects {
        let x = obj.x;
        let y = obj.y;
        let bw = obj.width;
        let bh = obj.height;

        for t in 0..thickness {
            let yt = y + t;
            let yb = y + bh - thickness + t;
            if yt >= 0 && yt < h {
                for dx in 0..bw {
                    let xx = x + dx;
                    if xx >= 0 && xx < w {
                        set_pixel(&mut output, xx, yt, color);
                    }
                }
            }
            if yb >= 0 && yb < h {
                for dx in 0..bw {
                    let xx = x + dx;
                    if xx >= 0 && xx < w {
                        set_pixel(&mut output, xx, yb, color);
                    }
                }
            }
        }

        for dy in 0..bh {
            let yy = y + dy;
            if yy >= 0 && yy < h {
                for t in 0..thickness {
                    let xl = x + t;
                    let xr = x + bw - thickness + t;
                    if xl >= 0 && xl < w {
                        set_pixel(&mut output, xl, yy, color);
                    }
                    if xr >= 0 && xr < w {
                        set_pixel(&mut output, xr, yy, color);
                    }
                }
            }
        }
    }

    output
}

fn set_pixel(image: &mut RgbImage, x: i32, y: i32, color: (u8, u8, u8)) {
    let idx = ((y * image.width as i32 + x) * 3) as usize;
    if idx + 2 < image.data.len() {
        image.data[idx] = color.0;
        image.data[idx + 1] = color.1;
        image.data[idx + 2] = color.2;
    }
}
