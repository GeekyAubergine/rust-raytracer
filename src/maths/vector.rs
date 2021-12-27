use nalgebra::Vector3;
use rand::Rng;

pub fn is_vector3f32_near_zero(vec: &Vector3<f32>) -> bool {
    const EPS: f32 = 1.0e-8;
    return vec.x.abs() < EPS && vec.y.abs() < EPS && vec.z.abs() < EPS;
}

pub fn random_point_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    loop {
        let point = Vector3::<f32>::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}

#[allow(dead_code)]
pub fn random_point_on_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    loop {
        let point = Vector3::<f32>::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if point.magnitude_squared() < 1.0 {
            return point.normalize();
        }
    }
}

#[allow(dead_code)]
pub fn random_point_in_unit_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let mut in_unit_sphere = random_point_in_unit_sphere();
    if in_unit_sphere.dot(normal) <= 0.0 {
        in_unit_sphere *= -1.0;
    }
    return in_unit_sphere;
}

pub fn random_point_in_unit_disk() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    loop {
        let point = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}
