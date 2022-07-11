use std::sync::Arc;

use glam::Vec3A;
use uuid::Uuid;

use crate::material::materials::Material;

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }
}

pub struct RayCollision {
    point: Vec3A,
    normal: Vec3A,
    time: f32,
    on_front_face: bool,
    material: Arc<Material>,
    bvh_node_uuids: Vec<Uuid>,
}

impl RayCollision {
    pub fn new(
        point: Vec3A,
        normal: Vec3A,
        time: f32,
        ray: &Ray,
        material: Arc<Material>,
    ) -> RayCollision {
        let front_face = ray.direction.dot(normal) < 0.0;
        let mut normal = normal;
        if !front_face {
            normal *= -1.0;
        }
        RayCollision {
            point,
            normal,
            time,
            on_front_face: front_face,
            material: material.clone(),
            bvh_node_uuids: Vec::new(),
        }
    }
    pub fn add_bvh_node_uuid(&mut self, node_uuid: Uuid) {
        self.bvh_node_uuids.push(node_uuid);
    }
    pub fn point(&self) -> Vec3A {
        self.point
    }
    pub fn normal(&self) -> Vec3A {
        self.normal
    }
    pub fn time(&self) -> f32 {
        self.time
    }
    pub fn on_front_face(&self) -> bool {
        self.on_front_face
    }
    pub fn material(&self) -> &Material {
        self.material.as_ref()
    }
    pub fn bvh_node_uuids(&self) -> &Vec<Uuid> {
        &self.bvh_node_uuids
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

    Some(root)
}
