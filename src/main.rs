use camera::Camera;
use glam::Vec3A;
use raytracer::render_scene_save_to_file;
use scene::generator::make_random_balls_scene;
use stats::Stats;
use std::{sync::Arc, thread};

use crossbeam_channel::Sender;
use ui::{pixel::PixelBatchUpdate, window::Window};

mod bounding_box;
mod camera;
mod color;
mod file;
mod material;
mod maths;
mod ray;
mod raytracer;
mod scene;
mod stats;
mod shape;
mod ui;

const IMAGE_WIDTH: u32 = 1080;
const IMAGE_HEIGHT: u32 = 920;
const WINDOW_WIDTH: u32 = 1080;
const WINDOW_HEIGHT: u32 = 920;
const THREAD_POOL_SIZE: u32 = 12;

const SAMPLES_PER_PIXEL_SIDE_VALUES: [u32; 4] = [1, 2, 4, 8];

fn ray_trace(width: u32, height: u32, pixel_batch_sender: Sender<PixelBatchUpdate>) {
    let camera = Camera::new(
        width,
        height,
        Vec3A::new(13.0, 2.0, 3.0),
        Vec3A::new(0.0, 0.0, 0.0),
        Vec3A::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        0.0,
    );
    let scene = Arc::new(make_random_balls_scene());
    let stats = Stats::new(pixel_batch_sender.clone(), 1);
    stats.clone().init();

    for samples_per_pixel_side in SAMPLES_PER_PIXEL_SIDE_VALUES {
        render_scene_save_to_file(
            &scene,
            &camera,
            samples_per_pixel_side,
            String::from("output/raytracer.png"),
            pixel_batch_sender.clone(),
            stats.clone(),
        );
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(THREAD_POOL_SIZE as usize)
        .build_global()
        .unwrap();

    let (pixel_batch_update_sender, pixel_batch_update_receiver) =
        crossbeam_channel::unbounded::<PixelBatchUpdate>();

    thread::spawn(|| {
        ray_trace(IMAGE_WIDTH, IMAGE_HEIGHT, pixel_batch_update_sender);
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
        pixel_batch_update_receiver,
    );

    let ui_result = window.init();

    match ui_result {
        Ok(_) => {}
        Err(error) => panic!("Error creating window {:?}", error),
    };
}
