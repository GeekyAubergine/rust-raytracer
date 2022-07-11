use std::sync::Arc;

use crossbeam_channel::Sender;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    file::save_png_from_pixel_data,
    bounding_box::bvh::BVHNode,
    ray::{Ray, RayCollider},
    color::Color,
    scene::Scene,
    stats::Stats,
    ui::pixel::{Pixel, PixelBatchUpdate, PixelsData}, shape::Sphere,
};

use super::camera::Camera;

const MAX_RAY_DEPTH: u32 = 64;
const CHUNK_SIZE: u32 = 5;

struct PixelChunk {
    y: u32,
    x: u32,
    chunk_size: u32,
}

fn ray_color(bvh_tree: &BVHNode, scene: &Arc<Scene>, ray: &Ray, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }

    if let Some(ray_collision) = bvh_tree.collide_ray(ray, 0.001, f32::INFINITY) {
        if let Some(material_scatter) = ray_collision.material().scatter(ray, &ray_collision) {
            material_scatter.color
                * ray_color(bvh_tree, scene, &material_scatter.ray, depth - 1)
        } else {
            Color::zero()
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn sample_pixel(
    bvh_tree: &BVHNode,
    scene: &Arc<Scene>,
    camera: &Camera,
    x: u32,
    y: u32,
    samples_per_pixel_side: u32,
    max_ray_depth: u32,
) -> Color {
    let mut pixel_color = Color::zero();

    for u_offset in 0..samples_per_pixel_side {
        for v_offset in 0..samples_per_pixel_side {
            let u_delta = u_offset as f32 / samples_per_pixel_side as f32;
            let v_delta = v_offset as f32 / samples_per_pixel_side as f32;

            // println!("{} {}", u_delta, v_delta);

            let u = (x as f32 + u_delta) / ((camera.screen_width() - 1) as f32);
            let v = (((camera.screen_height() - 1) as f32) - (y as f32) + v_delta)
                / ((camera.screen_height() - 1) as f32);

            let ray = camera.make_ray(u, v);

            let sample_color = ray_color(bvh_tree, scene, &ray, max_ray_depth);

            pixel_color += sample_color;
        }
    }

    pixel_color / (samples_per_pixel_side * samples_per_pixel_side) as f32
}

pub fn render_scene(
    scene: &Arc<Scene>,
    camera: &Camera,
    samples_per_pixel_side: u32,
    pixel_batch_sender: Sender<PixelBatchUpdate>,
    stats: Stats,
) -> PixelsData {
    let height = camera.screen_height();
    let width = camera.screen_width();
    let mut pixels: PixelsData = Vec::new();

    for y in 0..height {
        let mut row: Vec<Pixel> = Vec::new();
        for x in 0..width {
            row.push(Pixel::new(x, y, Color::zero()));
        }
        pixels.push(row);
    }

    let mut pixel_chunks: Vec<PixelChunk> = Vec::new();

    for x in 0..(width / CHUNK_SIZE) {
        for y in 0..(height / CHUNK_SIZE) {
            pixel_chunks.push(PixelChunk {
                y: y * CHUNK_SIZE,
                x: x * CHUNK_SIZE,
                chunk_size: CHUNK_SIZE,
            })
        }
    }

    let samples_per_pixel = samples_per_pixel_side * samples_per_pixel_side;

    let bvh_tree = BVHNode::build_tree(scene.colliders.clone(), 0.0, 1.0);

    stats
        .clone()
        .start_current_frame(pixel_chunks.len() as u32, samples_per_pixel);

    let pixel_updates: Vec<Pixel> = pixel_chunks
        .into_par_iter()
        .map(|chunk| {
            let mut pixel_updates: Vec<Pixel> = Vec::new();
            for y_offset in 0..chunk.chunk_size {
                for x_offset in 0..chunk.chunk_size {
                    let x = x_offset + chunk.x;
                    let y = y_offset + chunk.y;
                    let pixel_color = sample_pixel(
                        &bvh_tree,
                        scene,
                        camera,
                        x,
                        y,
                        samples_per_pixel_side,
                        MAX_RAY_DEPTH,
                    );

                    let pixel = Pixel::new(x as u32, y as u32, pixel_color);

                    pixel_updates.push(pixel);
                }
            }

            pixel_batch_sender
                .send(PixelBatchUpdate {
                    pixels: pixel_updates.clone(),
                })
                .unwrap();

            stats.clone().complete_chunk();

            pixel_updates
        })
        .reduce(
            Vec::new,
            |acc: Vec<Pixel>, arr: Vec<Pixel>| {
                let mut out = acc;

                for el in arr {
                    out.push(el);
                }

                out
            },
        );

    stats.complete_frame();

    for pixel in pixel_updates {
        pixels[pixel.position().y as usize][pixel.position().x as usize] = pixel;
    }

    pixels
}

pub fn render_scene_save_to_file(
    scene: &Arc<Scene>,
    camera: &Camera,
    samples_per_pixel_side: u32,
    file_path: String,
    pixel_batch_sender: Sender<PixelBatchUpdate>,
    stats: Stats,
) -> PixelsData {
    let pixel_data = render_scene(
        scene,
        camera,
        samples_per_pixel_side,
        pixel_batch_sender,
        stats,
    );
    save_png_from_pixel_data(file_path, &pixel_data);
    pixel_data
}
