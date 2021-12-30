use glam::Vec3A;
use rand::Rng;

pub fn is_Vec3Af32_near_zero(vec: &Vec3A) -> bool {
    const EPS: f32 = 1.0e-8;
    return vec.x.abs() < EPS && vec.y.abs() < EPS && vec.z.abs() < EPS;
}

pub fn random_point_in_unit_sphere() -> Vec3A {
    let mut rng = rand::thread_rng();

    loop {
        let point = Vec3A::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if point.length_squared() < 1.0 {
            return point;
        }
    }
}

#[allow(dead_code)]
pub fn random_point_on_unit_sphere() -> Vec3A {
    let mut rng = rand::thread_rng();

    loop {
        let point = Vec3A::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if point.length_squared() < 1.0 {
            return point.normalize();
        }
    }
}

#[allow(dead_code)]
pub fn random_point_in_unit_hemisphere(normal: Vec3A) -> Vec3A {
    let mut in_unit_sphere = random_point_in_unit_sphere();
    if in_unit_sphere.dot(normal) <= 0.0 {
        in_unit_sphere *= -1.0;
    }
    return in_unit_sphere;
}

pub fn random_point_in_unit_disk() -> Vec3A {
    let mut rng = rand::thread_rng();

    loop {
        let point = Vec3A::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

        if point.length_squared() < 1.0 {
            return point;
        }
    }
}
