use crate::settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;
use crate::renderer::shader::vertex_shader::ty::TransformationData;
use nalgebra_glm::{U3, Vec3, Mat4};
use nalgebra_glm as glm;
use nalgebra_glm::Dimension;

const MAX_PITCH: f32 = 89.0;
const MIN_PITCH: f32 = -89.0;

const MIN_FOV: f32 = 1.0;
const MAX_FOV: f32 = 45.0;

/// Struct representing a camera.
#[derive(Clone)]
pub struct Camera {
    settings: Rc<RefCell<Settings>>,
    position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,

    yaw: f32,
    pitch: f32,

    fov: f32,
}

impl Camera {
    /// Creates a new camera with default settings.
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        let position = Vec3::new(0.0, 0.0, 3.0);
        let front = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let world_up = up;
        let right = glm::normalize(&glm::cross::<f32, U3>(&front, &world_up));

        Camera {
            settings,
            position,
            front,
            up,
            right,
            world_up,
            yaw: -90.0,
            pitch: 0.0,
            fov: 45.0,
        }
    }

    /// Returns camera position
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Sets camera position
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vec3::new(x, y, z);
    }

    /// Sets camera position using Vec3
    pub fn set_position_vec3(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Returns camera yaw.
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    /// Sets camera yaw.
    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;

        self.update_camera_vectors();
    }

    /// Returns camera pitch
    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    /// Sets camera pitch.
    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = if pitch > MAX_PITCH {
            MAX_PITCH
        } else if pitch < MIN_PITCH {
            MIN_PITCH
        } else {
            pitch
        };

        self.update_camera_vectors();
    }

    /// Returns camera field of view.
    pub fn fov(&self) -> f32 {
        self.fov
    }

    /// Sets camera field of view.
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = if fov > MAX_FOV {
            MAX_FOV
        } else if fov < MIN_FOV {
            MIN_FOV
        } else {
            fov
        };
    }

    fn update_camera_vectors(&mut self) {
        self.front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.front.y = self.pitch.to_radians().sin();
        self.front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = glm::normalize(&self.front);
        self.right = glm::normalize(&glm::cross::<f32, U3>(&self.front, &self.world_up));
    }

    /// Returns camera view matrix.
    pub fn get_view_matrix(&self) -> Mat4 {
        glm::look_at(&self.position, &(&self.position + &self.front), &self.up)
    }

    /// Returns camera projection matrix.
    pub fn get_projection_matrix(&self) -> Mat4 {
        let (window_size, near_plane, far_plane) = {
            let settings = self.settings.borrow();
            (
                settings.window_size().clone(),
                settings.near_plane(),
                settings.far_plane(),
            )
        };
        let aspect_ratio = (window_size.width / window_size.height) as f32;
        glm::perspective(aspect_ratio, self.fov, near_plane, far_plane)
    }

    /// Returns model, view and projection matrix as uniform data. 
    /// Model should be updated with model matrix from Object.
    pub fn as_uniform_data(&self) -> TransformationData {
        TransformationData {
            model: Mat4::identity().into(),
            view: self.get_view_matrix().into(),
            proj: self.get_projection_matrix().into(),
        }
    }
}