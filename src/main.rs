use std::{sync::Arc, thread};

use crossbeam_channel::Sender;
use nalgebra::Vector3;
use render::{camera::Camera, raytracer::render_scene_save_to_file};
use scene::generator::make_random_balls_scene;
use ui::{
    pixel::{Pixel, PixelBatchUpdate, PixelsData},
    window::Window,
};

mod file;
mod geom;
mod maths;
mod ray;
mod render;
mod scene;
mod ui;

const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = 900;
const WINDOW_WIDTH: u32 = IMAGE_WIDTH;
const WINDOW_HEIGHT: u32 = IMAGE_HEIGHT;
const THREAD_POOL_SIZE: u32 = 12;

const SAMPLES_PER_PIXEL_SIDE_VALUES: [u32; 4] = [1, 2, 4, 8];

fn ray_trace(
    width: u32,
    height: u32,
    _pixel_sender: Sender<Pixel>,
    pixel_batch_sender: Sender<PixelBatchUpdate>,
    pixels_data_sender: Sender<PixelsData>,
) {
    let camera = Camera::new(
        width,
        height,
        Vector3::<f32>::new(13.0, 2.0, 3.0),
        Vector3::<f32>::new(0.0, 0.0, 0.0),
        Vector3::<f32>::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        0.0,
    );
    let scene = Arc::new(make_random_balls_scene());
    for samples_per_pixel_side in SAMPLES_PER_PIXEL_SIDE_VALUES {
        let pixel_batch_sender = pixel_batch_sender.clone();
        let pixels_data = render_scene_save_to_file(
            &scene,
            &camera,
            samples_per_pixel_side,
            String::from("output/raytracer.png"),
            pixel_batch_sender,
        );

        pixels_data_sender.send(pixels_data).unwrap();
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(THREAD_POOL_SIZE as usize)
        .build_global()
        .unwrap();

    let (pixel_sender, pixel_receiver) = crossbeam_channel::unbounded::<Pixel>();
    let (pixel_batch_update_sender, pixel_batch_update_receiver) =
        crossbeam_channel::unbounded::<PixelBatchUpdate>();
    let (pixels_data_sender, pixels_data_receiver) = crossbeam_channel::unbounded::<PixelsData>();

    thread::spawn(|| {
        ray_trace(
            IMAGE_WIDTH,
            IMAGE_HEIGHT,
            pixel_sender,
            pixel_batch_update_sender,
            pixels_data_sender,
        );
    });

    // thread::spawn(|| {
    //     let pixel_data = julia::generate_julia_pixel_data(
    //         IMAGE_WIDTH,
    //         IMAGE_HEIGHT,
    //         -0.4,
    //         0.6,
    //         pixel_sender,
    //         pixel_batch_update_sender,
    //         pixels_data_sender,
    //     );
    //     renderer::file::save_png_from_pixel_data(String::from("output/julia.png"), &pixel_data);
    // });

    let window = Window::new(
        WINDOW_WIDTH as f64,
        WINDOW_HEIGHT as f64,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        pixel_receiver,
        pixel_batch_update_receiver,
        pixels_data_receiver,
    );

    let ui_result = window.init();

    match ui_result {
        Ok(_) => {}
        Err(error) => panic!("Error creating window {:?}", error),
    };
}
