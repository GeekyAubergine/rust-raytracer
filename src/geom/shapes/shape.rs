use crate::{
    geom::bvh::aabb::{BoundingBox, AABB},
    ray::{
        ray::Ray,
        ray_collider::{RayCollider, RayCollision},
    },
};

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
