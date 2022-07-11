use crate::{shape::Sphere, bounding_box::{bvh::BVHNode, Aabb}, ray::{Ray, RayCollision}};

pub trait Collidable {
    fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision>;
    fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> Aabb;
}

pub enum Collider {
    SphereCollider(Sphere),
    BVHNodeCollider(BVHNode),
}


impl Collider {
    pub fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision> {
        match self {
            Collider::SphereCollider(sphere) => sphere.collide_ray(ray, t_min, t_max),
            Collider::BVHNodeCollider(bvh_node) => bvh_node.collide_ray(ray, t_min, t_max),
        }
    }
    pub fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> Aabb {
        match self {
            Collider::SphereCollider(sphere) => sphere.get_bounding_box(frame_start_time, frame_end_time),
            Collider::BVHNodeCollider(bvh_node) => bvh_node.get_bounding_box(frame_start_time, frame_end_time),
        }
    }
}
