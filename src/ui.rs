pub mod pixel {
    use nalgebra::Point2;

    use crate::color::Color;

    #[derive(Clone, Copy)]
    pub struct Pixel {
        position: Point2<u32>,
        color: Color,
    }

    impl Pixel {
        pub fn new(x: u32, y: u32, color: Color) -> Pixel {
            Pixel {
                position: Point2::<u32>::new(x, y),
                color,
            }
        }
        pub fn position(&self) -> Point2<u32> {
            self.position
        }
        pub fn color(&self) -> Color {
            self.color
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
                out[y as usize].push(Pixel::new(x, y, Color::zero()));
            }
        }

        out
    }
}

pub mod text {
    use font8x8::{UnicodeFonts, BASIC_FONTS, GREEK_FONTS, LATIN_FONTS};

    use crate::color::Color;

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

        if let Some(glyph) = data {
            for (y, row) in glyph.iter().enumerate() {
                for x in 0..8 {
                    let pixel = row >> x & 1;
                    let x = x * 2;
                    let y = y * 2;
                    if pixel == 1 {
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
                }
            }
        }

        pixels
    }

    fn render_string_line_at_position(x_pos: usize, y_pos: usize, string: &str) -> Vec<Pixel> {
        let mut background_pixels: Vec<Pixel> = Vec::new();

        let char_count = string.chars().count();

        for y in 0..(GLYPH_HEIGHT + GLYPH_PADDING_VERTICAL + GLYPH_PADDING_VERTICAL) {
            for x in 0..((GLYPH_WIDTH + GLYPH_PADDING_HORIZONTAL + GLYPH_PADDING_HORIZONTAL)
                * char_count)
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
                let x = x_pos
                    + (GLYPH_WIDTH + GLYPH_PADDING_HORIZONTAL + GLYPH_PADDING_HORIZONTAL) * index;
                render_char_at_position(
                    x + GLYPH_PADDING_HORIZONTAL,
                    y_pos + GLYPH_PADDING_VERTICAL,
                    char,
                )
            })
            .reduce(|acc: Vec<Pixel>, arr: Vec<Pixel>| {
                let mut out = acc;

                for el in arr {
                    out.push(el);
                }

                out
            });

        match pixels_opt {
            Some(glyph_pixels) => {
                let mut pixels = background_pixels.clone();
                for pixel in glyph_pixels {
                    pixels.push(pixel);
                }
                pixels
            }
            None => background_pixels,
        }
    }

    pub fn render_string_at_position(
        x_pos: usize,
        y_pos: usize,
        string: String,
    ) -> PixelBatchUpdate {
        let pixels_opt = string
            .split('\n')
            .enumerate()
            .map(|(index, line)| {
                let y = y_pos
                    + (GLYPH_HEIGHT + GLYPH_PADDING_VERTICAL + GLYPH_PADDING_VERTICAL) * index;
                render_string_line_at_position(x_pos, y, line)
            })
            .reduce(|acc: Vec<Pixel>, arr: Vec<Pixel>| {
                let mut out = acc;

                for el in arr {
                    out.push(el);
                }

                out
            });

        match pixels_opt {
            Some(pixels) => PixelBatchUpdate { pixels },
            None => PixelBatchUpdate { pixels: Vec::new() },
        }
    }
}

pub mod window {
    use log::error;
    use std::time::Duration;

    use pixels::{Error, Pixels, SurfaceTexture};
    use winit::{
        dpi::LogicalSize,
        event::{Event, VirtualKeyCode},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use winit_input_helper::WinitInputHelper;

    use super::pixel::PixelBatchUpdate;

    pub struct Window {
        window_width: f64,
        window_height: f64,
        texture_width: u32,
        texture_height: u32,
        pixel_update_batch_receiver: crossbeam_channel::Receiver<PixelBatchUpdate>,
    }

    impl Window {
        pub fn new(
            window_width: f64,
            window_height: f64,
            texture_width: u32,
            texture_height: u32,
            pixel_update_batch_receiver: crossbeam_channel::Receiver<PixelBatchUpdate>,
        ) -> Window {
            Window {
                window_width,
                window_height,
                texture_width,
                texture_height,
                pixel_update_batch_receiver,
            }
        }
        pub fn init(self) -> Result<(), Error> {
            let event_loop = EventLoop::new();
            let mut input = WinitInputHelper::new();
            let window = {
                let size = LogicalSize::new(self.window_width, self.window_height);
                WindowBuilder::new()
                    .with_title("Physics Simulations")
                    .with_inner_size(size)
                    .with_min_inner_size(size)
                    .build(&event_loop)
                    .unwrap()
            };

            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            let mut frame_pixels =
                Pixels::new(self.texture_width, self.texture_height, surface_texture)?;

            event_loop.run(move |event, _, control_flow| {
                // Draw the current frame
                if let Event::RedrawRequested(_) = event {
                    if frame_pixels
                        .render()
                        .map_err(|e| error!("pixels.render() failed: {}", e))
                        .is_err()
                    {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                {
                    let mut pixels_updated = false;
                    loop {
                        let pixel_result = self
                            .pixel_update_batch_receiver
                            .recv_timeout(Duration::from_micros(1));
                        match pixel_result {
                            Ok(pixels_batch_update) => {
                                for pixel_data in pixels_batch_update.pixels.iter() {
                                    let pixel_index =
                                        ((pixel_data.position().y * self.texture_width
                                            + pixel_data.position().x)
                                            * 4) as usize;
                                    let color = pixel_data.color();
                                    frame_pixels.get_frame()[pixel_index] =
                                        (color.r() * 255.0) as u8;
                                    frame_pixels.get_frame()[pixel_index + 1] =
                                        (color.g() * 255.0) as u8;
                                    frame_pixels.get_frame()[pixel_index + 2] =
                                        (color.b() * 255.0) as u8;
                                    frame_pixels.get_frame()[pixel_index + 3] = 255;
                                    pixels_updated = true;
                                }
                            }
                            Err(_) => {
                                if pixels_updated {
                                    window.request_redraw();
                                }
                                break;
                            }
                        }
                    }
                }

                // Handle input events
                if input.update(&event) {
                    // Close events
                    if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    // Resize the window
                    if let Some(size) = input.window_resized() {
                        frame_pixels.resize_surface(size.width, size.height);
                    }

                    // Update internal state and request a redraw
                    window.request_redraw();

                    // now = Instant::now();
                }
            });
        }
    }
}
