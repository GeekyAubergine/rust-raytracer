use crate::{
    ray::{ray::Ray, ray_collider::RayCollision},
    render::color::Color,
};

pub struct MaterialCollisionResult {
    pub color: Color,
    pub ray: Ray,
}
pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, collision: &RayCollision) -> Option<MaterialCollisionResult>;
}
