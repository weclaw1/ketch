use nalgebra_glm::{Vec3, Mat4};
use nalgebra_glm as glm;

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const FOV: f32 = 45.0;

pub struct Camera {
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
    pub fn new() -> Self {
        let position = Vec3::new(0.0, 0.0, 0.0);
        let front = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let world_up = up;
        let right = glm::normalize(&glm::cross(&front, &world_up));

        Camera {
            position,
            front,
            up,
            right,
            world_up,
            yaw: YAW,
            pitch: PITCH,
            fov: FOV,
        }
    }
}