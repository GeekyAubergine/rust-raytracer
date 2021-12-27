use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};

use image::Rgba;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        return Color { r, b, g, a };
    }
    pub fn zero() -> Color {
        return Color::new(0.0, 0.0, 0.0, 0.0);
    }
    pub fn rgba(self) -> Rgba<u8> {
        return image::Rgba([
            (255.0 * self.r) as u8,
            (255.0 * self.g) as u8,
            (255.0 * self.b) as u8,
            255 as u8,
        ]);
    }
    pub fn r(&self) -> &f32 {
        return &self.r;
    }
    pub fn g(&self) -> &f32 {
        return &self.g;
    }
    pub fn b(&self) -> &f32 {
        return &self.b;
    }
    pub fn a(&self) -> &f32 {
        return &self.a;
    }
    pub fn random(range: Range<f32>) -> Color {
        let mut rng = rand::thread_rng();

        return Color {
            r: rng.gen_range(range.clone()),
            b: rng.gen_range(range.clone()),
            g: rng.gen_range(range.clone()),
            a: rng.gen_range(range.clone()),
        };
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) -> () {
        *self = Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) -> () {
        *self = Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a * other,
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, other: f32) -> () {
        *self = Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
            a: self * other.a,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, other: f32) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
            a: self.a / other,
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, other: f32) -> () {
        *self = Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
            a: self.a / other,
        }
    }
}
