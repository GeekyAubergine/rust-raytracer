use nalgebra::Vector3;

use crate::{
    ray::{ray::Ray, ray_collider::RayCollision},
    render::color::Color, maths::vector::{random_point_in_unit_sphere, is_vector3f32_near_zero},
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
        let mut scatter_direction: Vector3<f32> =
            ray_collision.normal + random_point_in_unit_sphere().normalize();

        if is_vector3f32_near_zero(&scatter_direction) {
            scatter_direction = ray_collision.normal;
        }

        let scattered = Ray::new(ray_collision.point, scatter_direction, 0.0);

        Some(MaterialCollisionResult {
            color: self.albedo,
            ray: scattered,
        })
    }
}
