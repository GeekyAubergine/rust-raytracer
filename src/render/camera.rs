use nalgebra::Vector3;

#[derive(Clone, Copy, Debug)]
struct CameraSettings {
    screen_width: u32,
    screen_height: u32,
    camera_position: Vector3<f32>,
    look_at_position: Vector3<f32>,
    field_of_view: f32,
    aspect_ratio: f32,
}

pub struct Camera {
 settings: CameraSettings,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        camera_position: Vector3<f32>,
        look_at_position: Vector3<f32>,
        field_of_view: f32,
    ) -> Camera {
        return Camera {
            settings: CameraSettings {
                screen_width: width,
                screen_height: height,
                camera_position,
                look_at_position,
                field_of_view,
                aspect_ratio: width as f32 / height as f32,
            },
        };
    }
    pub fn screen_width(&self) -> u32 {
        return self.settings.screen_width;
    }
    pub fn screen_height(&self) -> u32 {
        return self.settings.screen_height;
    }
    pub fn position(&self) -> Vector3<f32> {
        return self.settings.camera_position;
    }
    pub fn look_at(&self) -> Vector3<f32> {
        return self.settings.look_at_position;
    }
    pub fn aspect_ratio(&self) -> f32 {
        return self.settings.aspect_ratio;
    }
    pub fn field_of_view(&self) -> f32 {
        return self.settings.field_of_view;
    }
    pub fn set_camera_position(&self, camera_position: Vector3<f32>) {
        let mut settings = self.settings;
        settings.camera_position = camera_position;
    }
    pub fn set_look_at(&self, look_at: Vector3<f32>) {
        let mut settings = self.settings;
        settings.look_at_position = look_at;
    }
}
