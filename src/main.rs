use image::ColorType;

fn main() {
    println!("Hello, world!");

    let (width, height): (usize, usize) = if cfg!(debug_assertions) {
        (160, 90)
    } else {
        (1920, 1080)
    };
    let mut buffer = vec![0; (width * height * 4) as usize];

    draw(&mut buffer, width, height);

    write_image(&buffer, width as u32, height as u32);
}

fn draw(buffer: &mut [u8], width: usize, height: usize) {
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

fn write_image(data: &[u8], width: u32, height: u32) {
    assert_eq!(data.len(), (width * height * 4) as usize);
    image::save_buffer("./img.png", data, width, height, ColorType::Rgba8).expect("failed to write image");
}
