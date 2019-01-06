use crate::settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;
use crate::renderer::shader::vertex_shader::ty::TransformationData;
use nalgebra_glm::{U3, Vec3, Mat4};
use nalgebra_glm as glm;

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
    pub fn position(&self) -> (f32, f32, f32) {
        (self.position.x, self.position.y, self.position.z)
    }

    /// Returns camera position as Vec3
    pub fn position_vec3(&self) -> Vec3 {
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

    /// Moves camera in chosen direction
    pub fn move_camera(&mut self, direction: Direction, value: f32) {
        let change_vector = match direction {
            Direction::Up    => value * self.front,
            Direction::Down  => -(value * self.front),
            Direction::Left  => -(value * glm::normalize(&glm::cross::<f32, U3>(&self.front, &self.up))),
            Direction::Right => value * glm::normalize(&glm::cross::<f32, U3>(&self.front, &self.up)),
        };

        self.position += change_vector;
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
    pub fn view_matrix(&self) -> Mat4 {
        glm::look_at(&self.position, &(&self.position + &self.front), &self.up)
    }

    /// Returns camera projection matrix.
    pub fn projection_matrix(&self) -> Mat4 {
        let (window_size, near_plane, far_plane) = {
            let settings = self.settings.borrow();
            (
                settings.window_size().clone(),
                settings.near_plane(),
                settings.far_plane(),
            )
        };
        let aspect_ratio = (window_size.width / window_size.height) as f32;

        let correction_matrix: Mat4 = Mat4::new(1.0, 0.0, 0.0, 0.0,
                                                0.0,-1.0, 0.0, 0.0,
                                                0.0, 0.0, 0.5, 0.0,
                                                0.0, 0.0, 0.5, 1.0);

        let proj_matrix = glm::perspective(aspect_ratio, self.fov, near_plane, far_plane);

        return correction_matrix * proj_matrix;
    }

    /// Returns model, view and projection matrix as uniform data. 
    /// Model should be updated with model matrix from Object.
    pub fn as_uniform_data(&self) -> TransformationData {
        TransformationData {
            model: Mat4::identity().into(),
            view: self.view_matrix().into(),
            proj: self.projection_matrix().into(),
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_camera_left() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let value = 2.0;
        let front = camera.front;
        let up = camera.up;
        let mut position = Vec3::new(0.0, 0.0, 3.0);

        camera.set_position_vec3(position);
        camera.move_camera(Direction::Left, value);

        let change_vector = -(value * glm::normalize(&glm::cross::<f32, U3>(&front, &up)));
        position += change_vector;

        assert_eq!(position, camera.position_vec3());
    }

    #[test]
    fn test_move_camera_right() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let value = 2.0;
        let front = camera.front;
        let up = camera.up;
        let mut position = Vec3::new(0.0, 0.0, 3.0);

        camera.set_position_vec3(position);
        camera.move_camera(Direction::Right, value);

        let change_vector = value * glm::normalize(&glm::cross::<f32, U3>(&front, &up));
        position += change_vector;

        assert_eq!(position, camera.position_vec3());
    }

    #[test]
    fn test_move_camera_up() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let value = 2.0;
        let front = camera.front;
        let mut position = Vec3::new(0.0, 0.0, 3.0);

        camera.set_position_vec3(position);
        camera.move_camera(Direction::Up, value);

        let change_vector = value * front;
        position += change_vector;

        assert_eq!(position, camera.position_vec3());
    }

    #[test]
    fn test_move_camera_down() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let value = 2.0;
        let front = camera.front;
        let mut position = Vec3::new(0.0, 0.0, 3.0);

        camera.set_position_vec3(position);
        camera.move_camera(Direction::Down, value);

        let change_vector = -(value * front);
        position += change_vector;

        assert_eq!(position, camera.position_vec3());
    }

    #[test]
    fn changing_position_changes_view_matrix() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let position = Vec3::new(0.0, 0.0, 3.0);
        camera.set_position_vec3(position);
        let view_matrix = camera.view_matrix();

        let new_position = Vec3::new(-10.0, 2.0, 3.0);
        camera.set_position_vec3(new_position);
        let new_view_matrix = camera.view_matrix();

        assert_ne!(view_matrix, new_view_matrix);
    }

    #[test]
    fn changing_yaw_changes_view_matrix() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let yaw = 5.0;
        camera.set_yaw(yaw);
        let view_matrix = camera.view_matrix();

        let new_yaw = -4.0;
        camera.set_yaw(new_yaw);
        let new_view_matrix = camera.view_matrix();

        assert_ne!(view_matrix, new_view_matrix);
    }

    #[test]
    fn changing_pitch_changes_view_matrix() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let pitch = 5.0;
        camera.set_pitch(pitch);
        let view_matrix = camera.view_matrix();

        let new_pitch = -4.0;
        camera.set_pitch(new_pitch);
        let new_view_matrix = camera.view_matrix();

        assert_ne!(view_matrix, new_view_matrix);
    }

    #[test]
    fn changing_fov_changes_projection_matrix() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let fov = 5.0;
        camera.set_fov(fov);
        let projection_matrix = camera.projection_matrix();

        let new_fov = 10.0;
        camera.set_fov(new_fov);
        let new_projection_matrix = camera.projection_matrix();

        assert_ne!(projection_matrix, new_projection_matrix);
    }

    #[test]
    fn if_pitch_is_greater_than_max_pitch_set_pitch_to_max_pitch() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let pitch = MAX_PITCH + 1.0;
        camera.set_pitch(pitch);

        assert_eq!(MAX_PITCH, camera.pitch());
    }

        #[test]
    fn if_pitch_is_less_than_min_pitch_set_pitch_to_min_pitch() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let pitch = MIN_PITCH - 1.0;
        camera.set_pitch(pitch);

        assert_eq!(MIN_PITCH, camera.pitch());
    }

    #[test]
    fn if_fov_is_greater_than_max_fov_set_fov_to_max_fov() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let fov = MAX_FOV + 1.0;
        camera.set_fov(fov);

        assert_eq!(MAX_FOV, camera.fov());
    }

    #[test]
    fn if_fov_is_less_than_min_fov_set_fov_to_min_fov() {
        let settings = Settings::new("test", 800.0, 600.0);
        let mut camera = Camera::new(Rc::new(RefCell::new(settings)));

        let fov = MIN_FOV - 1.0;
        camera.set_fov(fov);

        assert_eq!(MIN_FOV, camera.fov());
    }

}