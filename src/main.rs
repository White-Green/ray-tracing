use image::ColorType;

use crate::ray_tracing::draw;

pub mod ray_tracing;
pub mod geometry;

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

fn write_image(data: &[u8], width: u32, height: u32) {
    assert_eq!(data.len(), (width * height * 4) as usize);
    image::save_buffer("./img.png", data, width, height, ColorType::Rgba8).expect("failed to write image");
}
