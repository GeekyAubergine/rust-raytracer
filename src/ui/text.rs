use font8x8::{UnicodeFonts, BASIC_FONTS, GREEK_FONTS, LATIN_FONTS};

use crate::render::color::Color;

use super::pixel::{Pixel, PixelBatchUpdate};

const GLYPH_HEIGHT: usize = 16;
const GLYPH_WIDTH: usize = 16;
const GLYPH_PADDING_VERTICAL: usize = 2;
const GLYPH_PADDING_HORIZONTAL: usize = 2;

fn render_char_at_position(x_pos: usize, y_pos: usize, char: char) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();

    let mut data: Option<[u8; 8]> = BASIC_FONTS.get(char);

    if data.is_none() {
        data = LATIN_FONTS.get(char);
    }

    if data.is_none() {
        data = GREEK_FONTS.get(char);
    }

    match data {
        Some(glyph) => {
            for y in 0..glyph.len() {
                let row = glyph[y];
                for x in 0..8 {
                    let pixel = row >> x & 1;
                    let x = x * 2;
                    let y = y * 2;
                    match pixel {
                        1 => {
                            pixels.push(Pixel::new(
                                (x + x_pos) as u32,
                                (y + y_pos) as u32,
                                Color::one(),
                            ));
                            pixels.push(Pixel::new(
                                (x + 1 + x_pos) as u32,
                                (y + y_pos) as u32,
                                Color::one(),
                            ));
                            pixels.push(Pixel::new(
                                (x + x_pos) as u32,
                                (y + 1 + y_pos) as u32,
                                Color::one(),
                            ));
                            pixels.push(Pixel::new(
                                (x + 1 + x_pos) as u32,
                                (y + 1 + y_pos) as u32,
                                Color::one(),
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
        None => {}
    }

    return pixels;
}

fn render_string_line_at_position(x_pos: usize, y_pos: usize, string: &str) -> Vec<Pixel> {
    let mut background_pixels: Vec<Pixel> = Vec::new();

    let char_count = string.chars().count();

    for y in 0..(GLYPH_HEIGHT + GLYPH_PADDING_VERTICAL + GLYPH_PADDING_VERTICAL) {
        for x in
            0..((GLYPH_WIDTH + GLYPH_PADDING_HORIZONTAL + GLYPH_PADDING_HORIZONTAL) * char_count)
        {
            background_pixels.push(Pixel::new(
                (x + x_pos) as u32,
                (y + y_pos) as u32,
                Color::zero(),
            ));
        }
    }

    let pixels_opt = string
        .chars()
        .enumerate()
        .map(|(index, char)| {
            let x =
                x_pos + (GLYPH_WIDTH + GLYPH_PADDING_HORIZONTAL + GLYPH_PADDING_HORIZONTAL) * index;
            return render_char_at_position(
                x + GLYPH_PADDING_HORIZONTAL,
                y_pos + GLYPH_PADDING_VERTICAL,
                char,
            );
        })
        .reduce(|acc: Vec<Pixel>, arr: Vec<Pixel>| {
            let mut out = acc.clone();

            for el in arr {
                out.push(el);
            }

            return out;
        });

    match pixels_opt {
        Some(glyph_pixels) => {
            let mut pixels = background_pixels.clone();
            for pixel in glyph_pixels {
                pixels.push(pixel);
            }
            return pixels;
        }
        None => {
            return background_pixels;
        }
    }
}

pub fn render_string_at_position(x_pos: usize, y_pos: usize, string: String) -> PixelBatchUpdate {
    let pixels_opt = string
        .split("\n")
        .enumerate()
        .map(|(index, line)| {
            let y =
                y_pos + (GLYPH_HEIGHT + GLYPH_PADDING_VERTICAL + GLYPH_PADDING_VERTICAL) * index;
            return render_string_line_at_position(x_pos, y, line);
        })
        .reduce(|acc: Vec<Pixel>, arr: Vec<Pixel>| {
            let mut out = acc.clone();

            for el in arr {
                out.push(el);
            }

            return out;
        });

    match pixels_opt {
        Some(pixels) => {
            return PixelBatchUpdate { pixels };
        }
        None => {
            return PixelBatchUpdate { pixels: Vec::new() };
        }
    }
}
