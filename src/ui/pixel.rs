use nalgebra::Point2;

use crate::render::color::Color;

#[derive(Clone, Copy)]
pub struct Pixel {
    position: Point2<u32>,
    color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Pixel {
        return Pixel {
            position: Point2::<u32>::new(x, y),
            color,
        };
    }
    pub fn position(&self) -> Point2<u32> {
        return self.position;
    }
    pub fn color(&self) -> Color {
        return self.color;
    } 
}

pub type PixelsData = Vec<Vec<Pixel>>;

pub struct PixelBatchUpdate {
    pub pixels: Vec<Pixel>,
}

#[allow(dead_code)]
pub fn make_empty_pixel_data(width: u32, height: u32) -> PixelsData {
    let mut out: PixelsData = Vec::new();

    for y in 0..height {
        out.push(Vec::new());
        for x in 0..width {
            out[y as usize].push(Pixel::new(x, y, Color::new(0.0, 0.0, 0.0, 0.0)));
        }
    }

    return out;
}
