use crate::resource::mesh::Mesh;
use nalgebra_glm::{Mat4, Vec3};

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static ID_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct Object<'a> {
    id: u32,
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

    translation_matrix: Mat4,
    rotation_matrix: Mat4,
    scaling_matrix: Mat4, 

    model_matrix: Mat4,

    mesh: Option<&'a Mesh>,
}


impl<'a> Object<'a> {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn set_position_x(&mut self, position_x: f32) {
        self.position_x = position_x;
        self.update_translation_matrix();
        self.update_model_matrix();
    }

    pub fn set_position_y(&mut self, position_y: f32) {
        self.position_y = position_y;
        self.update_translation_matrix();
        self.update_model_matrix();
    }

    pub fn set_position_z(&mut self, position_z: f32) {
        self.position_z = position_z;
        self.update_translation_matrix();
        self.update_model_matrix();
    }

    pub fn set_position(&mut self, position_x: f32, position_y: f32, position_z: f32) {
        self.position_x = position_x;
        self.position_y = position_y;
        self.position_z = position_z;
        self.update_translation_matrix();
        self.update_model_matrix();
    }

    pub fn set_rotation_angle_x(&mut self, rotation_angle_x: f32) {
        self.rotation_angle_x = rotation_angle_x;
        self.update_rotation_matrix();
        self.update_model_matrix();
    }

    pub fn set_rotation_angle_y(&mut self, rotation_angle_y: f32) {
        self.rotation_angle_y = rotation_angle_y;
        self.update_rotation_matrix();
        self.update_model_matrix();
    }

    pub fn set_rotation_angle_z(&mut self, rotation_angle_z: f32) {
        self.rotation_angle_z = rotation_angle_z;
        self.update_rotation_matrix();
        self.update_model_matrix();
    }

    pub fn set_rotation_angles(&mut self, rotation_angle_x: f32, rotation_angle_y: f32, rotation_angle_z: f32) {
        self.rotation_angle_x = rotation_angle_x;
        self.rotation_angle_y = rotation_angle_y;
        self.rotation_angle_z = rotation_angle_z;
        self.update_rotation_matrix();
        self.update_model_matrix();
    }

    pub fn set_scale_x(&mut self, scale_x: f32) {
        self.scale_x = scale_x;
        self.update_scaling_matrix();
        self.update_model_matrix();
    }

    pub fn set_scale_y(&mut self, scale_y: f32) {
        self.scale_y = scale_y;
        self.update_scaling_matrix();
        self.update_model_matrix();
    }

    pub fn set_scale_z(&mut self, scale_z: f32) {
        self.scale_z = scale_z;
        self.update_scaling_matrix();
        self.update_model_matrix();
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale_x = scale;
        self.scale_y = scale;
        self.scale_z = scale;
        self.update_scaling_matrix();
        self.update_model_matrix();
    }

    pub fn set_scale_xyz(&mut self, scale_x: f32, scale_y: f32, scale_z: f32) {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self.scale_z = scale_z;
        self.update_scaling_matrix();
        self.update_model_matrix();
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

    pub fn position(&self) -> (f32, f32, f32) {
        (self.position_x, self.position_y, self.position_z)
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

    pub fn rotation_angles(&self) -> (f32, f32, f32) {
        (self.rotation_angle_x, self.rotation_angle_y, self.rotation_angle_z)
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

    pub fn scale(&self) -> (f32, f32, f32) {
        (self.scale_x, self.scale_y, self.scale_z)
    }

    pub fn model_matrix(&self) -> Mat4 {
        self.model_matrix
    }

    pub fn mesh(&self) -> Option<&Mesh> {
        self.mesh
    }

    pub fn set_mesh(&mut self, mesh: &'a Mesh) {
        self.mesh = Some(mesh);
    }

    fn update_translation_matrix(&mut self) {
        self.translation_matrix = position_to_translation_matrix(self.position_x, self.position_y, self.position_z);
    }

    fn update_rotation_matrix(&mut self) {
        self.rotation_matrix = rotation_of_axes_to_rotation_matrix(self.rotation_angle_x, self.rotation_angle_y, self.rotation_angle_z);
    }

    fn update_scaling_matrix(&mut self) {
        self.scaling_matrix = scale_of_axes_to_scale_matrix(self.scale_x, self.scale_y, self.scale_z);
    }

    fn update_model_matrix(&mut self) {
        self.model_matrix = create_model_matrix(&self.translation_matrix, &self.rotation_matrix, &self.scaling_matrix);
    }
}

impl<'a> Clone for Object<'a> {
    fn clone(&self) -> Object<'a> {
        Object {
            id: generate_id(),
            name: self.name.clone(),

            position_x: self.position_x,
            position_y: self.position_y,
            position_z: self.position_z,

            rotation_angle_x: self.rotation_angle_x,
            rotation_angle_y: self.rotation_angle_y,
            rotation_angle_z: self.rotation_angle_z,

            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,

            translation_matrix: self.translation_matrix,
            rotation_matrix: self.rotation_matrix,
            scaling_matrix: self.scaling_matrix,

            model_matrix: self.model_matrix,

            mesh: self.mesh    
        }
    }
}

pub struct ObjectBuilder<'a> {
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

    translation_matrix: Mat4,
    rotation_matrix: Mat4,
    scaling_matrix: Mat4, 

    mesh: Option<&'a Mesh>,
}

impl<'a> ObjectBuilder<'a> {
    pub fn new<S: Into<String>>(name: S) -> Self {
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

            translation_matrix: Mat4::identity(),
            rotation_matrix: Mat4::identity(),
            scaling_matrix: Mat4::identity(),

            mesh: None,        
        }
    }

    pub fn with_mesh(self, mesh: &'a Mesh) -> Self {
        Self {
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

            translation_matrix: self.translation_matrix,
            rotation_matrix: self.rotation_matrix,
            scaling_matrix: self.scaling_matrix,

            mesh: Some(mesh),  
        }
    }

    pub fn with_position(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            name: self.name,

            position_x: x,
            position_y: y,
            position_z: z,

            rotation_angle_x: self.rotation_angle_x,
            rotation_angle_y: self.rotation_angle_y,
            rotation_angle_z: self.rotation_angle_z,

            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,

            translation_matrix: position_to_translation_matrix(x, y, z),
            rotation_matrix: self.rotation_matrix,
            scaling_matrix: self.scaling_matrix,

            mesh: self.mesh,        
        }
    }

    pub fn with_rotation_angle(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            name: self.name,

            position_x: self.position_x,
            position_y: self.position_y,
            position_z: self.position_z,

            rotation_angle_x: x,
            rotation_angle_y: y,
            rotation_angle_z: z,

            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,

            translation_matrix: self.translation_matrix,
            rotation_matrix: rotation_of_axes_to_rotation_matrix(x, y, z),
            scaling_matrix: self.scaling_matrix,

            mesh: self.mesh,         
        }
    }

    pub fn with_scale(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            name: self.name,

            position_x: self.position_x,
            position_y: self.position_y,
            position_z: self.position_z,

            rotation_angle_x: self.rotation_angle_x,
            rotation_angle_y: self.rotation_angle_y,
            rotation_angle_z: self.rotation_angle_z,

            scale_x: x,
            scale_y: y,
            scale_z: z,

            translation_matrix: self.translation_matrix,
            rotation_matrix: self.rotation_matrix,
            scaling_matrix: scale_of_axes_to_scale_matrix(x, y, z),

            mesh: self.mesh,        
        }
    }

    pub fn build(&self) -> Object<'a> {
        Object {
            id: generate_id(),
            name: self.name.clone(),

            position_x: self.position_x,
            position_y: self.position_y,
            position_z: self.position_z,

            rotation_angle_x: self.rotation_angle_x,
            rotation_angle_y: self.rotation_angle_y,
            rotation_angle_z: self.rotation_angle_z,

            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,

            translation_matrix: self.translation_matrix,
            rotation_matrix: self.rotation_matrix,
            scaling_matrix: self.scaling_matrix,

            model_matrix: create_model_matrix(&self.translation_matrix, &self.rotation_matrix, &self.scaling_matrix),

            mesh: self.mesh,  
        }
    }
}

fn position_to_translation_matrix(position_x: f32, position_y: f32, position_z: f32) -> Mat4 {
    nalgebra_glm::translate(&nalgebra_glm::identity(), &Vec3::new(position_x, position_y, position_z))
}

fn rotation_of_axes_to_rotation_matrix(rotation_angle_x: f32, rotation_angle_y: f32, rotation_angle_z: f32) -> Mat4 {
    let rotation_matrix_z = nalgebra_glm::rotate_z(&nalgebra_glm::identity(), rotation_angle_z);
    let rotation_matrix_y = nalgebra_glm::rotate_y(&nalgebra_glm::identity(), rotation_angle_y);
    let rotation_matrix_x = nalgebra_glm::rotate_x(&nalgebra_glm::identity(), rotation_angle_x);

    rotation_matrix_z * rotation_matrix_y * rotation_matrix_x
}

fn scale_of_axes_to_scale_matrix(scale_x: f32, scale_y: f32, scale_z: f32) -> Mat4 {
    nalgebra_glm::scale(&nalgebra_glm::identity(), &Vec3::new(scale_x, scale_y, scale_z))
}

fn create_model_matrix(translation_matrix: &Mat4, rotation_matrix: &Mat4, scaling_matrix: &Mat4) -> Mat4 {
    translation_matrix * rotation_matrix * scaling_matrix
}

fn generate_id() -> u32 {
    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    id as u32
}