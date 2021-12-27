use crate::{
    ray::{ray::Ray, ray_collider::RayCollision},
    render::color::Color,
};

use super::{
    material::{Material, MaterialCollisionResult},
    utils::reflect_ray,
};

pub struct Metal {
    albedo: Color,
    smoothness: f32,
}

impl Metal {
    pub fn new(albedo: Color, smoothness: f32) -> Metal {
        return Metal { albedo, smoothness };
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &RayCollision) -> Option<MaterialCollisionResult> {
        let scattered = reflect_ray(ray, hit_record.point, hit_record.normal, self.smoothness);
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            return Some(MaterialCollisionResult {
                color: self.albedo,
                ray: scattered,
            });
        }
        return None;
    }
}
