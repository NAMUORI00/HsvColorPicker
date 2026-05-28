pub fn dilate(mask: &[u8], width: u32, height: u32, kernel_size: u32, iterations: u32) -> Vec<u8> {
    let w = width as i32;
    let h = height as i32;
    let pad = (kernel_size / 2) as i32;
    let len = (width * height) as usize;
    let mut result = mask.to_vec();

    for _ in 0..iterations {
        let mut temp = vec![0u8; len];
        for y in 0..h {
            for x in 0..w {
                'kernel: for ky in (y - pad).max(0)..=(y + pad).min(h - 1) {
                    for kx in (x - pad).max(0)..=(x + pad).min(w - 1) {
                        if result[(ky * w + kx) as usize] > 0 {
                            temp[(y * w + x) as usize] = 255;
                            break 'kernel;
                        }
                    }
                }
            }
        }
        result = temp;
    }

    result
}
