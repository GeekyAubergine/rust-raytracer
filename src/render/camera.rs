use glam::Vec3A;

use crate::{
    maths::{random_f32_between, random_point_in_unit_disk},
    ray::ray::Ray,
};

#[derive(Clone, Copy, Debug)]
struct CameraSettings {
    screen_width: u32,
    screen_height: u32,
    camera_position: Vec3A,
    look_at_position: Vec3A,
    up_vector: Vec3A,
    field_of_view: f32,
    aspect_ratio: f32,
    aperture: f32,
    shutter: f32,
}

#[derive(Clone)]
struct CameraMetadata {
    horizontal: Vec3A,
    vertical: Vec3A,
    camera_u: Vec3A,
    camera_v: Vec3A,
    lower_left_corner: Vec3A,
    lens_radius: f32,
}

pub struct Camera {
    settings: CameraSettings,
    metadata: CameraMetadata,
}

fn recalculate_camera(settings: CameraSettings) -> Camera {
    let focus_distance = (settings.camera_position - settings.look_at_position).length();

    // Horizontal field-of-view in degrees
    let theta = std::f32::consts::PI / 180.0 * settings.field_of_view;
    let viewport_height = 2.0 * (theta / 2.0).tan();
    let viewport_width = settings.aspect_ratio * viewport_height;

    let cw = (settings.camera_position - settings.look_at_position).normalize();
    let camera_u = settings.up_vector.cross(cw).normalize();
    let camera_v = cw.cross(camera_u);

    let horizontal = focus_distance * viewport_width * camera_u;
    let vertical = focus_distance * viewport_height * camera_v;

    let lower_left_corner =
        settings.camera_position - horizontal / 2.0 - vertical / 2.0 - focus_distance * cw;
    let lens_radius = settings.aperture / 2.0;

    return Camera {
        settings,
        metadata: CameraMetadata {
            horizontal,
            vertical,
            camera_u,
            camera_v,
            lower_left_corner,
            lens_radius,
        },
    };
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        camera_position: Vec3A,
        look_at_position: Vec3A,
        up_vector: Vec3A,
        field_of_view: f32,
        aperture: f32,
        shutter: f32,
    ) -> Camera {
        return recalculate_camera(CameraSettings {
            screen_width: width,
            screen_height: height,
            camera_position,
            look_at_position,
            up_vector,
            field_of_view,
            aspect_ratio: width as f32 / height as f32,
            aperture,
            shutter,
        });
    }
    pub fn screen_width(&self) -> u32 {
        return self.settings.screen_width;
    }
    pub fn screen_height(&self) -> u32 {
        return self.settings.screen_height;
    }
    pub fn position(&self) -> Vec3A {
        return self.settings.camera_position;
    }
    pub fn look_at(&self) -> Vec3A {
        return self.settings.look_at_position;
    }
    pub fn aspect_ratio(&self) -> f32 {
        return self.settings.aspect_ratio;
    }
    pub fn field_of_view(&self) -> f32 {
        return self.settings.field_of_view;
    }
    pub fn set_camera_position(&mut self, camera_position: Vec3A) {
        let mut settings = self.settings;
        settings.camera_position = camera_position;
        *self = recalculate_camera(settings);
    }
    pub fn set_look_at(&mut self, look_at: Vec3A) {
        let mut settings = self.settings;
        settings.look_at_position = look_at;
        *self = recalculate_camera(settings);
    }
    pub fn make_ray(&self, u: f32, v: f32) -> Ray {
        let random_disk = self.metadata.lens_radius * random_point_in_unit_disk();
        let offset =
            self.metadata.camera_u * random_disk.x + self.metadata.camera_v * random_disk.y;

        return Ray::new(
            self.settings.camera_position + offset,
            self.metadata.lower_left_corner
                + self.metadata.horizontal * u
                + self.metadata.vertical * v
                - self.settings.camera_position
                - offset,
            random_f32_between(0.0, self.settings.shutter),
        );
    }
}
