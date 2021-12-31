use glam::Vec3A;

use crate::{
    ray::{ray::Ray, ray_collider::RayCollision},
    render::color::Color, maths::{random_point_in_unit_sphere, is_Vec3Af32_near_zero},
};

use super::material::{Material, MaterialCollisionResult};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        return Lambertian { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, ray_collision: &RayCollision) -> Option<MaterialCollisionResult> {
        let mut scatter_direction: Vec3A =
            ray_collision.normal() + random_point_in_unit_sphere().normalize();

        if is_Vec3Af32_near_zero(&scatter_direction) {
            scatter_direction = ray_collision.normal();
        }

        let scattered = Ray::new(ray_collision.point(), scatter_direction, 0.0);

        Some(MaterialCollisionResult {
            color: self.albedo,
            ray: scattered,
        })
    }
}
