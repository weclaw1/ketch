#[derive(Clone)]
pub struct Object<T> {
    name: String,

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
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn set_position_x(&mut self, position_x: f32) {
        self.position_x = position_x;
    }

    pub fn set_position_y(&mut self, position_y: f32) {
        self.position_y = position_y;
    }

    pub fn set_position_z(&mut self, position_z: f32) {
        self.position_z = position_z;
    }

    pub fn set_rotation_angle_x(&mut self, rotation_angle_x: f32) {
        self.rotation_angle_x = rotation_angle_x;
    }

    pub fn set_rotation_angle_y(&mut self, rotation_angle_y: f32) {
        self.rotation_angle_y = rotation_angle_y;
    }

    pub fn set_rotation_angle_z(&mut self, rotation_angle_z: f32) {
        self.rotation_angle_z = rotation_angle_z;
    }

    pub fn set_scale_x(&mut self, scale_x: f32) {
        self.scale_x = scale_x;
    }

    pub fn set_scale_y(&mut self, scale_y: f32) {
        self.scale_y = scale_y;
    }

    pub fn set_scale_z(&mut self, scale_z: f32) {
        self.scale_z = scale_z;
    }

    pub fn position_x(&self) -> f32 {
        self.position_x
    }

    pub fn position_y(&self) -> f32 {
        self.position_y
    }

    pub fn position_z(&self) -> f32 {
        self.position_z
    }

    pub fn rotation_angle_x(&self) -> f32 {
        self.rotation_angle_x
    }

    pub fn rotation_angle_y(&self) -> f32 {
        self.rotation_angle_y
    }

    pub fn rotation_angle_z(&self) -> f32 {
        self.rotation_angle_z
    }

    pub fn scale_x(&self) -> f32 {
        self.scale_x
    }

    pub fn scale_y(&self) -> f32 {
        self.scale_y
    }

    pub fn scale_z(&self) -> f32 {
        self.scale_z
    }

    pub fn inner(&self) -> &T {
        self.inner
    }
}

pub struct ObjectBuilder<T> {
    name: String,

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
    pub fn new<S: Into<String>>(name: S, inner: &T) -> Self {
        Self {
            name: name.into(),

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
            name: self.name,

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