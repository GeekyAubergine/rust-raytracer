use std::sync::Arc;

use crate::collider::Collider;

// type SyncedShaped = Shape;

type ArcCollider = Arc<Collider>;

#[derive(Clone)]
pub struct Scene {
    pub colliders: Vec<ArcCollider>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            colliders: Vec::new(),
        }
    }
    pub fn add_shape(&mut self, shape: ArcCollider) {
        self.colliders.push(shape)
    }
}

pub mod generator {
    use std::sync::Arc;

    use glam::Vec3A;
    use rand::Rng;

    use crate::{
        collider::Collider::SphereCollider,
        color::Color,
        material::materials::{
            Dielectric, Lambertian,
            Material::{MaterialDielectric, MaterialLambertian, MaterialMetal},
            Metal,
        },
        shape::Sphere,
    };

    use super::Scene;

    pub fn make_random_balls_scene() -> Scene {
        let mut scene = Scene::new();
        let mut rng = rand::thread_rng();

        let ground_mat = Arc::new(MaterialLambertian(Lambertian::new(Color::new(
            0.5, 0.5, 0.5,
        ))));
        let ground_sphere = Sphere::new(0.0, -1000.0, 0.0, 1000.0, ground_mat, Vec3A::ZERO);

        scene.add_shape(Arc::new(SphereCollider(ground_sphere)));

        let x = 11;

        for a in -x..=x {
            for b in -x..=x {
                let mat: f32 = rng.gen::<f32>();

                let center = Vec3A::new(
                    (a as f32) + rng.gen_range(0.0..0.9),
                    0.2,
                    (b as f32) + rng.gen_range(0.0..0.9),
                );

                if mat < 0.7 {
                    // Diffuse
                    let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    let sphere_mat = Arc::new(MaterialLambertian(Lambertian::new(albedo)));
                    let sphere = Sphere::new(
                        center.x,
                        center.y,
                        center.z,
                        0.2,
                        sphere_mat,
                        Vec3A::new(0.0, 0.1, 0.0),
                    );

                    scene.add_shape(Arc::new(SphereCollider(sphere)));
                } else if mat < 0.9 {
                    // Metal
                    let albedo = Color::random(0.4..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_mat = Arc::new(MaterialMetal(Metal::new(albedo, fuzz)));
                    let sphere =
                        Sphere::new(center.x, center.y, center.z, 0.2, sphere_mat, Vec3A::ZERO);

                    scene.add_shape(Arc::new(SphereCollider(sphere)));
                } else {
                    // Glass
                    let sphere_mat = Arc::new(MaterialDielectric(Dielectric::new(1.5, 0.8)));
                    let sphere =
                        Sphere::new(center.x, center.y, center.z, 0.2, sphere_mat, Vec3A::ZERO);

                    scene.add_shape(Arc::new(SphereCollider(sphere)));
                }
            }
        }

        let mat1 = Arc::new(MaterialDielectric(Dielectric::new(1.5, 0.8)));
        let mat2 = Arc::new(MaterialLambertian(Lambertian::new(Color::new(
            0.4, 0.2, 0.1,
        ))));
        let mat3 = Arc::new(MaterialMetal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
        let mat4 = Arc::new(MaterialDielectric(Dielectric::new(1.5, 1.0)));

        let sphere1 = Sphere::new(0.0, 1.0, 0.0, 1.0, mat1, Vec3A::ZERO);
        let sphere2 = Sphere::new(-4.0, 1.0, 0.0, 1.0, mat2, Vec3A::ZERO);
        let sphere3 = Sphere::new(4.0, 1.0, 0.0, 1.0, mat3, Vec3A::ZERO);
        let sphere4 = Sphere::new(8.0, 1.5, 2.0, 0.3, mat4, Vec3A::ZERO);

        scene.add_shape(Arc::new(SphereCollider(sphere1)));
        scene.add_shape(Arc::new(SphereCollider(sphere2)));
        scene.add_shape(Arc::new(SphereCollider(sphere3)));
        scene.add_shape(Arc::new(SphereCollider(sphere4)));

        scene
    }
}
