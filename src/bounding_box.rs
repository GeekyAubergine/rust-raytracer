use glam::Vec3A;

use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub minimum: Vec3A,
    pub maximum: Vec3A,
}

impl Aabb {
    pub fn new(minimum: Vec3A, maximum: Vec3A) -> Aabb {
        Aabb { minimum, maximum }
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

        true
    }
}

pub fn build_surrounding_bounding_box(box_a: Aabb, box_b: Aabb) -> Aabb {
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

    Aabb::new(minimum, maximum)
}

pub trait BoundingBox {
    fn get_bounding_box(&self, frame_start_time: f32, frame_end_time: f32) -> Aabb;
}

pub mod bvh {
    use std::{cmp::Ordering, sync::Arc};

    use glam::Vec3A;
    use uuid::Uuid;

    use crate::{
        maths::random_usize_between,
        ray::{Ray, RayCollision}, collider::Collider,
    };

    use super::{build_surrounding_bounding_box, Aabb};

    type ArcCollidable = Arc<Collider>;

    pub struct BVHNode {
        uuid: Uuid,
        left: ArcCollidable,
        right: ArcCollidable,
        pub aabb: Aabb,
    }

    fn bounding_box_minimum_for_shape(
        shape: &ArcCollidable,
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> Vec3A {
        shape.get_bounding_box(frame_start_time, frame_end_time).minimum
    }

    fn sort_aabb_by_x(
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> Box<dyn FnMut(&ArcCollidable, &ArcCollidable) -> Ordering> {
        Box::new(move |a, b| {
            let ax = bounding_box_minimum_for_shape(a, frame_start_time, frame_end_time).x;
            let bx = bounding_box_minimum_for_shape(b, frame_start_time, frame_end_time).x;
            if ax < bx {
                return Ordering::Less;
            }

            if ax == bx {
                return Ordering::Equal;
            }

            Ordering::Greater
        })
    }

    fn sort_aabb_by_y(
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> Box<dyn FnMut(&ArcCollidable, &ArcCollidable) -> Ordering> {
        Box::new(move |a, b| {
            let ay = bounding_box_minimum_for_shape(a, frame_start_time, frame_end_time).y;
            let by = bounding_box_minimum_for_shape(b, frame_start_time, frame_end_time).y;
            if ay < by {
                return Ordering::Less;
            }

            if ay == by {
                return Ordering::Equal;
            }

            Ordering::Greater
        })
    }

    fn sort_aabb_by_z(
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> Box<dyn FnMut(&ArcCollidable, &ArcCollidable) -> Ordering> {
        Box::new(move |a, b| {
            let az = bounding_box_minimum_for_shape(a, frame_start_time, frame_end_time).z;
            let bz = bounding_box_minimum_for_shape(b, frame_start_time, frame_end_time).z;
            if az < bz {
                return Ordering::Less;
            }

            if az == bz {
                return Ordering::Equal;
            }

            Ordering::Greater
        })
    }

    fn sub_divide_children_into_node(
        children: Vec<ArcCollidable>,
        frame_start_time: f32,
        frame_end_time: f32,
    ) -> BVHNode {
        if children.is_empty() {
            panic!("No children given",);
        }

        let mut left: Option<&ArcCollidable> = children.get(0);
        let mut right: Option<&ArcCollidable> = children.get(0);

        let mut comparator = match random_usize_between(0, 2) {
            0 => sort_aabb_by_x(frame_start_time, frame_end_time),
            1 => sort_aabb_by_y(frame_start_time, frame_end_time),
            2 => sort_aabb_by_z(frame_start_time, frame_end_time),
            _ => panic!("Unexpected axis value"),
        };

        match children.len().cmp(&2) {
            Ordering::Greater => {
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
                    Arc::new(Collider::BVHNodeCollider(left_node)),
                    Arc::new(Collider::BVHNodeCollider(right_node)),
                    aabb,
                );
            }
            Ordering::Equal => match comparator(&children[0], &children[1]) {
                Ordering::Greater => {
                    left = children.get(1);
                    right = children.get(0);
                }
                _ => {
                    left = children.get(0);
                    right = children.get(1);
                }
            },
            _ => {}
        }

        match left {
            None => panic!("Left node is null"),
            Some(actual_left) => match right {
                None => panic!("Left node is null"),
                Some(actual_right) => {
                    let box_left = actual_left.get_bounding_box(frame_start_time, frame_end_time);
                    let box_right = actual_right.get_bounding_box(frame_start_time, frame_end_time);

                    let aabb = build_surrounding_bounding_box(box_left, box_right);

                    BVHNode::new(
                        Uuid::new_v4(),
                        actual_left.clone(),
                        actual_right.clone(),
                        aabb,
                    )
                }
            },
        }
    }

    impl BVHNode {
        pub fn new(uuid: Uuid, left: ArcCollidable, right: ArcCollidable, aabb: Aabb) -> BVHNode {
            BVHNode {
                uuid,
                left,
                right,
                aabb,
            }
        }

        pub fn build_tree(
            children: Vec<ArcCollidable>,
            frame_start_time: f32,
            frame_end_time: f32,
        ) -> BVHNode {
            sub_divide_children_into_node(children, frame_start_time, frame_end_time)
        }
        pub fn collide_ray(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayCollision> {
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

            None
        }

        pub fn get_bounding_box(&self, _frame_start_time: f32, _frame_end_time: f32) -> Aabb {
            self.aabb
        }
    }
}
