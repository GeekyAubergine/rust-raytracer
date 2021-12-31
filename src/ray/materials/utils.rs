use glam::Vec3A;
use rand::Rng;

use crate::{maths::random_point_in_unit_sphere, ray::ray::Ray};


pub(crate) fn reflect_Vec3Af32(v1: Vec3A, v2: Vec3A) -> Vec3A {
    return v1 - 2.0 * v1.dot(v2) * v2;
}

pub(crate) fn reflect_ray(ray: &Ray, point: Vec3A, normal: Vec3A, smoothness: f32) -> Ray {
    let direction = reflect_Vec3Af32(ray.direction, normal); //ray.direction - 2.0 * ray.direction.dot(&normal) * normal;
    let roughness_offset = (1.0 - smoothness) * random_point_in_unit_sphere();
    return Ray::new(point + roughness_offset, direction, ray.time);
}

pub(crate) fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    // Use Schlick's approximation for reflectance
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
}

pub(crate) fn refract_ray(
    ray: &Ray,
    point: Vec3A,
    normal: Vec3A,
    on_front_face: bool,
    refraction_index: f32,
) -> Ray {
    let refraction_ratio: f32 = if on_front_face {
        1.0 / refraction_index
    } else {
        refraction_index
    };

    let unit_direction = ray.direction.normalize();

    let cos_theta = ((-1.0) * unit_direction).dot(normal).min(1.0);
    let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

    let mut rng = rand::thread_rng();
    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let will_reflect = rng.gen::<f32>() < reflectance(cos_theta, refraction_ratio);

    if cannot_refract || will_reflect {
        let direction = reflect_Vec3Af32(unit_direction, normal);
        return Ray::new(point, direction, 0.0);
    } else {
        let cos_theta = (-1.0 * unit_direction).dot(normal).min(1.0);
        let r_out_perpendicular = refraction_ratio * (unit_direction + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perpendicular.length_squared()).abs().sqrt() * normal;
        let direction = r_out_perpendicular + r_out_parallel;

        return Ray::new(point, direction, ray.time);
    }
}
