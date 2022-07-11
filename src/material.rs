use crate::{
    ray::{Ray},
    color::Color,
};

pub struct MaterialCollisionResult {
    pub color: Color,
    pub ray: Ray,
}

mod utils {
    use glam::Vec3A;
    use rand::Rng;

    use crate::{maths::random_point_in_unit_sphere, ray::Ray};

    pub(crate) fn reflect_vec3_af32(v1: Vec3A, v2: Vec3A) -> Vec3A {
        v1 - 2.0 * v1.dot(v2) * v2
    }

    pub(crate) fn reflect_ray(ray: &Ray, point: Vec3A, normal: Vec3A, smoothness: f32) -> Ray {
        let direction = reflect_vec3_af32(ray.direction, normal); //ray.direction - 2.0 * ray.direction.dot(&normal) * normal;
        let roughness_offset = (1.0 - smoothness) * random_point_in_unit_sphere();
        Ray::new(point + roughness_offset, direction, ray.time)
    }

    pub(crate) fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
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
            let direction = reflect_vec3_af32(unit_direction, normal);
            Ray::new(point, direction, 0.0)
        } else {
            let cos_theta = (-1.0 * unit_direction).dot(normal).min(1.0);
            let r_out_perpendicular = refraction_ratio * (unit_direction + cos_theta * normal);
            let r_out_parallel =
                -(1.0 - r_out_perpendicular.length_squared()).abs().sqrt() * normal;
            let direction = r_out_perpendicular + r_out_parallel;

            Ray::new(point, direction, ray.time)
        }
    }
}

pub mod materials {
    use glam::Vec3A;

    use crate::{
        maths::{is_vec3_af32_near_zero, random_point_in_unit_sphere},
        ray::{Ray, RayCollision},
        color::Color,
    };

    use super::{
        utils::{reflect_ray, refract_ray}, MaterialCollisionResult,
    };

    pub struct Dielectric {
        refraction_index: f32,
        transparency: f32,
    }

    impl Dielectric {
        pub fn new(refraction_index: f32, transparency: f32) -> Dielectric {
            Dielectric {
                refraction_index,
                transparency,
            }
        }
        pub fn scatter(&self, ray: &Ray, hit_record: &RayCollision) -> Option<MaterialCollisionResult> {
            Some(MaterialCollisionResult {
                color: Color::new(self.transparency, self.transparency, self.transparency),
                ray: refract_ray(
                    ray,
                    hit_record.point(),
                    hit_record.normal(),
                    hit_record.on_front_face(),
                    self.refraction_index,
                ),
            })
        }
    }

    pub struct Lambertian {
        albedo: Color,
    }

    impl Lambertian {
        pub fn new(albedo: Color) -> Lambertian {
            Lambertian { albedo }
        }
        pub fn scatter(
            &self,
            _ray: &Ray,
            ray_collision: &RayCollision,
        ) -> Option<MaterialCollisionResult> {
            let mut scatter_direction: Vec3A =
                ray_collision.normal() + random_point_in_unit_sphere().normalize();

            if is_vec3_af32_near_zero(&scatter_direction) {
                scatter_direction = ray_collision.normal();
            }

            let scattered = Ray::new(ray_collision.point(), scatter_direction, 0.0);

            Some(MaterialCollisionResult {
                color: self.albedo,
                ray: scattered,
            })
        }
    }

    pub struct Metal {
        albedo: Color,
        smoothness: f32,
    }

    impl Metal {
        pub fn new(albedo: Color, smoothness: f32) -> Metal {
            Metal { albedo, smoothness }
        }
        pub fn scatter(&self, ray: &Ray, hit_record: &RayCollision) -> Option<MaterialCollisionResult> {
            let scattered = reflect_ray(
                ray,
                hit_record.point(),
                hit_record.normal(),
                self.smoothness,
            );
            if scattered.direction.dot(hit_record.normal()) > 0.0 {
                return Some(MaterialCollisionResult {
                    color: self.albedo,
                    ray: scattered,
                });
            }
            None
        }
    }

    pub enum Material {
        MaterialDielectric(Dielectric),
        MaterialLambertian(Lambertian),
        MaterialMetal(Metal),
    }

    impl Material {
        pub fn scatter(&self, ray: &Ray, collision: &RayCollision) -> Option<MaterialCollisionResult> {
            match self {
                Material::MaterialDielectric(dielectric) => dielectric.scatter(ray, collision),
                Material::MaterialLambertian(lambertian) => lambertian.scatter(ray, collision),
                Material::MaterialMetal(metal) => metal.scatter(ray, collision),
            }
        }
    }
}
