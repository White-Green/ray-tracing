pub mod scene;

pub fn draw(buffer: &mut [u8], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let yy = y as f64 - (height / 2) as f64;
            let xx = x as f64 - (width / 2) as f64;
            let radius = (width.min(height) / 2) as f64;
            if xx * xx + yy * yy <= radius * radius {
                buffer[(y * width + x) * 4] = 255;
                buffer[(y * width + x) * 4 + 3] = 255;
            }
        }
    }
}
