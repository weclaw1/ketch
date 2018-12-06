pub struct Object<T> {
    position_x: f32,
    position_y: f32,
    position_z: f32,

    rotation_angle_x: f32,
    rotation_angle_y: f32,
    rotation_angle_z: f32,

    scale_x: f32,
    scale_y: f32,
    scale_z: f32,

    inner: &T,
}

impl<T> Object<T> {
    pub fn new(inner: &T) {

    }
}

pub struct ObjectBuilder<T> {
    position_x: f32,
    position_y: f32,
    position_z: f32,

    rotation_angle_x: f32,
    rotation_angle_y: f32,
    rotation_angle_z: f32,

    scale_x: f32,
    scale_y: f32,
    scale_z: f32,

    inner: &T,
}

impl<T> ObjectBuilder<T> {
    pub fn new(inner: &T) -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,

            rotation_angle_x: 0.0,
            rotation_angle_y: 0.0,
            rotation_angle_z: 0.0,

            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,

            inner         
        }
    }

    pub fn with_position(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            position_x: x,
            position_y: y,
            position_z: z,

            rotation_angle_x: self.rotation_angle_x,
            rotation_angle_y: self.rotation_angle_y,
            rotation_angle_z: self.rotation_angle_z,

            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,

            inner: self.inner         
        }
    }

    pub fn with_rotation_angle(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            position_x: self.position_x,
            position_y: self.position_y,
            position_z: self.position_z,

            rotation_angle_x: x,
            rotation_angle_y: y,
            rotation_angle_z: z,

            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,

            inner: self.inner         
        }
    }

    pub fn with_scale(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            position_x: self.position_x,
            position_y: self.position_y,
            position_z: self.position_z,

            rotation_angle_x: self.rotation_angle_x,
            rotation_angle_y: self.rotation_angle_y,
            rotation_angle_z: self.rotation_angle_z,

            scale_x: x,
            scale_y: y,
            scale_z: z,

            inner: self.inner         
        }
    }

    pub fn build(&self) -> Object<T> {
        Object<T> {
            position_x: self.position_x,
            position_y: self.position_y,
            position_z: self.position_z,

            rotation_angle_x: self.rotation_angle_x,
            rotation_angle_y: self.rotation_angle_y,
            rotation_angle_z: self.rotation_angle_z,

            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,

            inner: self.inner    
        }
    }
}