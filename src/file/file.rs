use image::Rgba;

use crate::ui::pixel::PixelsData;

pub fn save_png_from_pixel_data(path: String, pixels: &PixelsData) {
    println!("Saving file");
    let height = pixels.len();

    if height < 1 {
        return;
    }

    let width = pixels[0].len();

    let mut image_buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::new(width as u32, height as u32);

    for y in 0..pixels.len() {
        let row = &pixels[y];
        for x in 0..row.len() {
            let color = pixels[y][x].color().rgba();
            image_buffer.put_pixel(x as u32, y as u32, color)
        }
    }

    let file_path = std::path::Path::new(&path);
    let prefix = file_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    image_buffer.save(path).unwrap();
    println!("Done");
}