use nalgebra::Vector3;

use crate::ray::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub minimum: Vector3<f32>,
    pub maximum: Vector3<f32>,
}

impl AABB {
    pub fn new(minimum: Vector3<f32>, maximum: Vector3<f32>) -> AABB {
        return AABB { minimum, maximum };
    }
    pub fn does_ray_collide(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_direction_x = 1.0 / ray.direction.x;
        let mut tx0 = (self.minimum.x - ray.origin.x) * inv_direction_x;
        let mut tx1 = (self.maximum.x - ray.origin.x) * inv_direction_x;

        if inv_direction_x < 0.0 {
            std::mem::swap(&mut tx0, &mut tx1);
        }

        let tx_min = if tx0 > t_min { tx0 } else { t_min };
        let tx_max = if tx1 < t_max { tx1 } else { t_max };

        if tx_max <= tx_min {
            return false;
        }

        let inv_direction_y = 1.0 / ray.direction.y;
        let mut ty0 = (self.minimum.y - ray.origin.y) * inv_direction_y;
        let mut ty1 = (self.maximum.y - ray.origin.y) * inv_direction_y;

        if inv_direction_y < 0.0 {
            std::mem::swap(&mut ty0, &mut ty1);
        }

        let ty_min = if ty0 > t_min { ty0 } else { t_min };
        let ty_max = if ty1 < t_max { ty1 } else { t_max };

        if ty_max <= ty_min {
            return false;
        }

        let inv_direction_z = 1.0 / ray.direction.z;
        let mut tz0 = (self.minimum.z - ray.origin.z) * inv_direction_z;
        let mut tz1 = (self.maximum.z - ray.origin.z) * inv_direction_z;

        if inv_direction_z < 0.0 {
            std::mem::swap(&mut tz0, &mut tz1);
        }

        let tz_min = if tz0 > t_min { tz0 } else { t_min };
        let tz_max = if tz1 < t_max { tz1 } else { t_max };

        if tz_max <= tz_min {
            return false;
        }

        return true;
    }
}

pub fn build_surrounding_bounding_box(box_a: AABB, box_b: AABB) -> AABB {
    let minimum = Vector3::<f32>::new(
        box_a.minimum.x.min(box_b.minimum.x),
        box_a.minimum.y.min(box_b.minimum.y),
        box_a.minimum.z.min(box_b.minimum.z),
    );
    let maximum = Vector3::<f32>::new(
        box_a.maximum.x.max(box_b.maximum.x),
        box_a.maximum.y.max(box_b.maximum.y),
        box_a.maximum.z.max(box_b.maximum.z),
    );

    return AABB::new(minimum, maximum)
}

pub trait BoundingBox {
    fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> AABB;
}
