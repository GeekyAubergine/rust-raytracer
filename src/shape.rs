use std::sync::Arc;

use glam::Vec3A;

use crate::{ray::{Ray, RayCollision, RayCollider, collide_ray_with_sphere}, material::materials::Material};

use super::bounding_box::{build_surrounding_bounding_box, BoundingBox, Aabb};

// pub trait Collidable {
//     fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision>;
//     fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> Aabb;
// }

// impl<T> RayCollider for T
// where
//     T: Collidable + Send + Sync,
// {
//     fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision> {
//         self.collide_ray(ray, t_min, t_max)
//     }
// }

// impl<T> BoundingBox for T
// where
//     T: Collidable + Send + Sync,
// {
//     fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> Aabb {
//         self.get_bounding_box(frame_start_time, frame_end_time)
//     }
// }

pub struct Sphere {
    pub centre: Vec3A,
    pub radius: f32,
    pub velocity: Vec3A,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        material: Arc<Material>,
        velocity: Vec3A,
    ) -> Sphere {
        Sphere {
            centre: Vec3A::new(x, y, z),
            radius: r,
            material,
            velocity,
        }
    }
    pub fn center_at_frame_time(&self, time_delta: f32) -> Vec3A {
        self.centre + self.velocity * time_delta
    }
    pub fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision> {
        let centre = self.center_at_frame_time(ray.time);
        let root = collide_ray_with_sphere(ray, centre, self.radius, t_min, t_max);

        match root {
            None => {
                None
            }
            Some(root) => {
                let hit_point = ray.at(root);
                let normal = (hit_point - centre) / self.radius;

                let hit = RayCollision::new(hit_point, normal, root, ray, self.material.clone());

                Some(hit)
            }
        }
    }

    pub fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> Aabb {
        let radius_vec = Vec3A::new(self.radius, self.radius, self.radius);
        let centre_start = self.center_at_frame_time(frame_start_time);
        let centre_end = self.center_at_frame_time(frame_end_time);

        let box_start = Aabb::new(centre_start - radius_vec, centre_start + radius_vec);
        let box_end = Aabb::new(centre_end - radius_vec, centre_end + radius_vec);

        build_surrounding_bounding_box(box_start, box_end)
    }
}
