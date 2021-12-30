use std::sync::Arc;

use glam::Vec3A;

use super::{materials::material::Material, ray::Ray};

pub struct RayCollision {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub time: f32,
    pub on_front_face: bool,
    pub material: Arc<dyn Material>,
}

impl RayCollision {
    pub fn new(
        point: Vec3A,
        normal: Vec3A,
        time: f32,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> RayCollision {
        let front_face = ray.direction.dot(normal) < 0.0;
        let mut normal = normal;
        if !front_face {
            normal *= -1.0;
        }
        return RayCollision {
            point,
            normal,
            time,
            on_front_face: front_face,
            material: material.clone(),
        };
    }
}

pub trait RayCollider: Send + Sync {
    fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision>;
}

pub fn collide_ray_with_sphere(
    ray: &Ray,
    centre: Vec3A,
    radius: f32,
    t_min: f32,
    t_max: f32,
) -> Option<f32> {
    // If pointing away from sphere ignore
    if (centre - ray.origin).dot(ray.direction) < 0.0 {
        return None;
    }

    let oc = ray.origin - centre;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = ray.origin.distance_squared(centre) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return None;
    }

    let discriminant_sqrt = discriminant.sqrt();

    let mut root = (-half_b - discriminant_sqrt) / a;
    if root < t_min || root >= t_max {
        root = (-half_b + discriminant_sqrt) / a;
        if root < t_min || root >= t_max {
            return None;
        }
    }

    return Some(root);
}
