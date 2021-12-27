use super::color::Color;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

impl Pixel {
    pub fn new(x: usize, y: usize, color: Color) -> Pixel {
        return Pixel { x, y, color };
    }
}

pub type PixelsData = Vec<Vec<Pixel>>;

pub struct PixelBatchUpdate {
    pub pixels: Vec<Pixel>,
}

#[allow(dead_code)]
pub fn make_empty_pixel_data(width: usize, height: usize) -> PixelsData {
    let mut out: PixelsData = Vec::new();

    for y in 0..height {
        out.push(Vec::new());
        for x in 0..width {
            out[y].push(Pixel::new(x, y, Color::new(0.0, 0.0, 0.0, 0.0)));
        }
    }

    return out;
}
