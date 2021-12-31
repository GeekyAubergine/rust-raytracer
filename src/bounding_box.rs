use glam::Vec3A;

use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub minimum: Vec3A,
    pub maximum: Vec3A,
}

impl AABB {
    pub fn new(minimum: Vec3A, maximum: Vec3A) -> AABB {
        return AABB { minimum, maximum };
    }
    pub fn does_ray_collide(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inverse_ray_direction = ray.direction.recip();
        let min_origin = self.minimum - ray.origin;
        let max_origin = self.maximum - ray.origin;

        let inv_direction_x = inverse_ray_direction.x;
        let mut tx0 = min_origin.x * inv_direction_x;
        let mut tx1 = max_origin.x * inv_direction_x;

        if inv_direction_x < 0.0 {
            std::mem::swap(&mut tx0, &mut tx1);
        }

        let tx_min = if tx0 > t_min { tx0 } else { t_min };
        let tx_max = if tx1 < t_max { tx1 } else { t_max };

        if tx_max <= tx_min {
            return false;
        }

        let inv_direction_y = inverse_ray_direction.y;
        let mut ty0 = min_origin.y * inv_direction_y;
        let mut ty1 = max_origin.y * inv_direction_y;

        if inv_direction_y < 0.0 {
            std::mem::swap(&mut ty0, &mut ty1);
        }

        let ty_min = if ty0 > t_min { ty0 } else { t_min };
        let ty_max = if ty1 < t_max { ty1 } else { t_max };

        if ty_max <= ty_min {
            return false;
        }

        let inv_direction_z = inverse_ray_direction.z;
        let mut tz0 = min_origin.z * inv_direction_z;
        let mut tz1 = max_origin.z * inv_direction_z;

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
    let minimum = Vec3A::new(
        box_a.minimum.x.min(box_b.minimum.x),
        box_a.minimum.y.min(box_b.minimum.y),
        box_a.minimum.z.min(box_b.minimum.z),
    );
    let maximum = Vec3A::new(
        box_a.maximum.x.max(box_b.maximum.x),
        box_a.maximum.y.max(box_b.maximum.y),
        box_a.maximum.z.max(box_b.maximum.z),
    );

    return AABB::new(minimum, maximum);
}

pub trait BoundingBox {
    fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> AABB;
}

pub mod bvh {
    use std::{cmp::Ordering, sync::Arc};

    use uuid::Uuid;

    use crate::{
        shape::Shape,
        maths::random_usize_between,
        ray::{Ray, RayCollision},
    };

    use super::{build_surrounding_bounding_box, AABB};

    type SyncedShaped = dyn Shape + Send + Sync;

    type ArcShape = Arc<SyncedShaped>;

    pub struct BVHNode {
        uuid: Uuid,
        left: ArcShape,
        right: ArcShape,
        pub aabb: AABB,
    }

    fn sort_aabb_by_x(
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> Box<dyn FnMut(&ArcShape, &ArcShape) -> Ordering> {
        return Box::new(move |a, b| {
            let ax = a
                .get_bounding_box(frame_start_time, frame_end_time)
                .minimum
                .x;
            let bx = b
                .get_bounding_box(frame_start_time, frame_end_time)
                .minimum
                .x;
            if ax < bx {
                return Ordering::Less;
            }

            if ax == bx {
                return Ordering::Equal;
            }

            return Ordering::Greater;
        });
    }

    fn sort_aabb_by_y(
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> Box<dyn FnMut(&ArcShape, &ArcShape) -> Ordering> {
        return Box::new(move |a, b| {
            let ay = a
                .get_bounding_box(frame_start_time, frame_end_time)
                .minimum
                .y;
            let by = b
                .get_bounding_box(frame_start_time, frame_end_time)
                .minimum
                .y;
            if ay < by {
                return Ordering::Less;
            }

            if ay == by {
                return Ordering::Equal;
            }

            return Ordering::Greater;
        });
    }

    fn sort_aabb_by_z(
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> Box<dyn FnMut(&ArcShape, &ArcShape) -> Ordering> {
        return Box::new(move |a, b| {
            let az = a
                .get_bounding_box(frame_start_time, frame_end_time)
                .minimum
                .z;
            let bz = b
                .get_bounding_box(frame_start_time, frame_end_time)
                .minimum
                .z;
            if az < bz {
                return Ordering::Less;
            }

            if az == bz {
                return Ordering::Equal;
            }

            return Ordering::Greater;
        });
    }

    fn sub_divide_children_into_node(
        children: Vec<ArcShape>,
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> BVHNode {
        if children.len() == 0 {
            panic!("No children given",);
        }

        let mut left: Option<&ArcShape> = children.get(0);
        let mut right: Option<&ArcShape> = children.get(0);

        let mut comparator = match random_usize_between(0, 2) {
            0 => sort_aabb_by_x(frame_start_time, frame_end_time),
            1 => sort_aabb_by_y(frame_start_time, frame_end_time),
            2 => sort_aabb_by_z(frame_start_time, frame_end_time),
            _ => panic!("Unexpected axis value"),
        };

        if children.len() > 2 {
            let mut sorted_children = children.clone();
            sorted_children.sort_by(comparator);

            let mid = sorted_children.len() / 2;
            let mut left_children = sorted_children;
            let right_children = left_children.split_off(mid);

            let left_node =
                sub_divide_children_into_node(left_children, frame_start_time, frame_end_time);
            let right_node =
                sub_divide_children_into_node(right_children, frame_start_time, frame_end_time);

            let box_left = left_node.get_bounding_box(frame_start_time, frame_end_time);
            let box_right = right_node.get_bounding_box(frame_start_time, frame_end_time);

            let aabb = build_surrounding_bounding_box(box_left, box_right);

            return BVHNode::new(
                Uuid::new_v4(),
                Arc::new(left_node),
                Arc::new(right_node),
                aabb,
            );
        } else if children.len() == 2 {
            match comparator(&children[0], &children[1]) {
                Ordering::Greater => {
                    left = children.get(1);
                    right = children.get(0);
                }
                _ => {
                    left = children.get(0);
                    right = children.get(1);
                }
            }
        }

        match left {
            None => panic!("Left node is null"),
            Some(actual_left) => match right {
                None => panic!("Left node is null"),
                Some(actual_right) => {
                    let box_left = actual_left.get_bounding_box(frame_start_time, frame_end_time);
                    let box_right = actual_right.get_bounding_box(frame_start_time, frame_end_time);

                    let aabb = build_surrounding_bounding_box(box_left, box_right);

                    return BVHNode::new(
                        Uuid::new_v4(),
                        actual_left.clone(),
                        actual_right.clone(),
                        aabb,
                    );
                }
            },
        }
    }

    impl BVHNode {
        pub fn new(uuid: Uuid, left: ArcShape, right: ArcShape, aabb: AABB) -> BVHNode {
            return BVHNode {
                uuid,
                left,
                right,
                aabb,
            };
        }

        pub fn build_tree(
            children: Vec<ArcShape>,
            frame_start_time: f32,
            frame_end_time: f32,
        ) -> BVHNode {
            return sub_divide_children_into_node(children, frame_start_time, frame_end_time);
        }
    }

    impl Shape for BVHNode {
        fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision> {
            let collides = self.aabb.does_ray_collide(ray, t_min, t_max);

            if collides {
                let left_collision = self.left.collide_ray(ray, t_min, t_max);

                let mut new_t_max = t_max;
                let left_collision_copy = left_collision.as_ref();
                match left_collision_copy {
                    None => {}
                    Some(left_collision) => {
                        new_t_max = left_collision.time();
                    }
                }
                let right_collision = self.right.collide_ray(ray, t_min, new_t_max);

                match right_collision {
                    None => {
                        return left_collision;
                    }
                    Some(right_collision) => {
                        return Some(right_collision);
                    }
                }
            }

            return None;
        }

        fn get_bounding_box(&self, _frame_start_time: f32, _frame_end_time: f32) -> AABB {
            return self.aabb;
        }
    }
}
