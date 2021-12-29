use glam::Vec3A;

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A, time: f32) -> Ray {
        return Ray {
            origin,
            direction,
            time,
        };
    }

    pub fn at(&self, t: f32) -> Vec3A {
        return self.origin + self.direction * t;
    }
}
