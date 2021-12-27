use std::sync::Arc;

use nalgebra::Vector3;
use rand::Rng;

use crate::{
    geom::shapes::sphere::Sphere,
    maths::vector::make_vector3f32_zero,
    ray::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    render::color::Color,
};

use super::scene::Scene;

pub fn make_random_balls_scene() -> Scene {
    let mut scene = Scene::new();
    let mut rng = rand::thread_rng();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5, 1.0)));
    let ground_sphere = Sphere::new(
        0.0,
        -1000.0,
        0.0,
        1000.0,
        ground_mat,
        make_vector3f32_zero(),
    );

    scene.add_shape(Arc::new(ground_sphere));

    let x = 11;

    for a in -x..=x {
        for b in -x..=x {
            let mat: f32 = rng.gen::<f32>();

            let center = Vector3::<f32>::new(
                (a as f32) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f32) + rng.gen_range(0.0..0.9),
            );

            if mat < 0.7 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(
                    center.x,
                    center.y,
                    center.z,
                    0.2,
                    sphere_mat,
                    Vector3::<f32>::new(0.0, 0.1, 0.0),
                );

                scene.add_shape(Arc::new(sphere));
            } else if mat < 0.9 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(
                    center.x,
                    center.y,
                    center.z,
                    0.2,
                    sphere_mat,
                    make_vector3f32_zero(),
                );

                scene.add_shape(Arc::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5, 0.8));
                let sphere = Sphere::new(
                    center.x,
                    center.y,
                    center.z,
                    0.2,
                    sphere_mat,
                    make_vector3f32_zero(),
                );

                scene.add_shape(Arc::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5, 0.8));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1, 1.0)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5, 1.0), 0.0));

    let sphere1 = Sphere::new(0.0, 1.0, 0.0, 1.0, mat1, make_vector3f32_zero());
    let sphere2 = Sphere::new(-4.0, 1.0, 0.0, 1.0, mat2, make_vector3f32_zero());
    let sphere3 = Sphere::new(4.0, 1.0, 0.0, 1.0, mat3, make_vector3f32_zero());

    scene.add_shape(Arc::new(sphere1));
    scene.add_shape(Arc::new(sphere2));
    scene.add_shape(Arc::new(sphere3));

    return scene;
}
