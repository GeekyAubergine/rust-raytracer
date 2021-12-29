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
        return Window {
            window_width,
            window_height,
            texture_width,
            texture_height,
            pixel_update_batch_receiver,
        };
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
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
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
                                let pixel_index = ((pixel_data.position().y * self.texture_width
                                    + pixel_data.position().x)
                                    * 4) as usize;
                                let color = pixel_data.color();
                                frame_pixels.get_frame()[pixel_index] = (color.r() * 255.0) as u8;
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
