use std::sync::Arc;

use glam::Vec3A;

use crate::{ray::{Ray, RayCollision, RayCollider, collide_ray_with_sphere}, material::Material};

use super::bounding_box::{build_surrounding_bounding_box, BoundingBox, AABB};

pub trait Shape {
    fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision>;
    fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> AABB;
}

impl<T> RayCollider for T
where
    T: Shape + Send + Sync,
{
    fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision> {
        return self.collide_ray(ray, t_min, t_max);
    }
}

impl<T> BoundingBox for T
where
    T: Shape + Send + Sync,
{
    fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> AABB {
        return self.get_bounding_box(frame_start_time, frame_end_time);
    }
}

pub struct Sphere {
    pub centre: Vec3A,
    pub radius: f32,
    pub velocity: Vec3A,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        material: Arc<dyn Material + Send + Sync>,
        velocity: Vec3A,
    ) -> Sphere {
        return Sphere {
            centre: Vec3A::new(x, y, z),
            radius: r,
            material,
            velocity,
        };
    }
    pub fn center_at_frame_time(&self, time_delta: f32) -> Vec3A {
        return self.centre + self.velocity * time_delta;
    }
}

impl Shape for Sphere {
    fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision> {
        let centre = self.center_at_frame_time(ray.time);
        let root = collide_ray_with_sphere(ray, centre, self.radius, t_min, t_max);

        match root {
            None => {
                return None;
            }
            Some(root) => {
                let hit_point = ray.at(root);
                let normal = (hit_point - centre) / self.radius;

                let hit = RayCollision::new(hit_point, normal, root, ray, self.material.clone());

                return Some(hit);
            }
        }
    }

    fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> AABB {
        let radius_vec = Vec3A::new(self.radius, self.radius, self.radius);
        let centre_start = self.center_at_frame_time(frame_start_time);
        let centre_end = self.center_at_frame_time(frame_end_time);

        let box_start = AABB::new(centre_start - radius_vec, centre_start + radius_vec);
        let box_end = AABB::new(centre_end - radius_vec, centre_end + radius_vec);

        return build_surrounding_bounding_box(box_start, box_end);
    }
}
