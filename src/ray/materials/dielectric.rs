use crate::{
    render::color::Color, ray::{ray::Ray, ray_collider::RayCollision},
};

use super::{
    material::{Material, MaterialCollisionResult},
    utils::refract_ray,
};

pub struct Dielectric {
    refraction_index: f32,
    transparency: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32, transparency: f32) -> Dielectric {
        return Dielectric {
            refraction_index,
            transparency,
        };
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &RayCollision) -> Option<MaterialCollisionResult> {
        return Some(MaterialCollisionResult {
            color: Color::new(self.transparency, self.transparency, self.transparency, 1.0),
            ray: refract_ray(
                ray,
                hit_record.point,
                hit_record.normal,
                hit_record.on_front_face,
                self.refraction_index,
            ),
        });
    }
}
