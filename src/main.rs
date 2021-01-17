use image::ColorType;

fn main() {
    println!("Hello, world!");

    let (width, height) = if cfg!(debug_assertions) {
        (160, 90)
    } else {
        (1920, 1080)
    };
    let mut buffer = vec![255; (width * height * 4) as usize];

    // draw to buffer in here

    write_image(&buffer, width, height);
}

fn write_image(data: &[u8], width: u32, height: u32) {
    assert_eq!(data.len(), (width * height * 4) as usize);
    image::save_buffer("./img.png", data, width, height, ColorType::Rgba8).expect("failed to write image");
}
