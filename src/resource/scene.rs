use crate::renderer::shader::fragment_shader::ty::LightData;
use nalgebra_glm::Vec3;
use crate::resource::object::Object;
use crate::resource::camera::Camera;

/// Scene is a collection of game objects and world properties.
/// Only one can be active at a time.
#[derive(Clone)]
pub struct Scene {
    name: String,

    camera: Camera,
    objects: Vec<Object>,
    light_position: Vec3,
    light_color: Vec3,
}

impl Scene {
    /// Creates new scene.
    pub fn new<S: Into<String>>(name: S, camera: Camera) -> Self {
        Scene {
            name: name.into(),
            camera,
            objects: Vec::new(),
            light_position: Vec3::new(0.0, 0.0, 0.0),
            light_color: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    /// Returns the name of this scene.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Adds object to the scene.
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    /// Removes object from the scene and returns it if found.
    pub fn remove_object(&mut self, id: u32) -> Option<Object> {
        let object_index = self.objects.iter().position(|x| x.id() == id);
        match object_index {
            Some(index) => Some(self.objects.remove(index)),
            None => None,
        }
    }

    /// Removes objects with specified name and returns them in a vector if any are found. 
    pub fn remove_objects_with_name(&mut self, name: &str) -> Vec<Object> {
        let object_indexes: Vec<usize> = self.objects.iter()
                                                     .enumerate()
                                                     .filter(|(i, x)| x.name() == name)
                                                     .map(|(i, x)| i)
                                                     .collect();

        let mut removed_objects = Vec::new();
        for index in object_indexes {
            removed_objects.push(self.objects.remove(index));
        }
        removed_objects
    }

    /// Returns a reference to slice of all objects.
    pub fn objects(&self) -> &[Object] {
        self.objects.as_slice()
    }

    /// Returns a reference to a mutable slice of all objects
    pub fn objects_mut(&mut self) -> &mut [Object] {
        self.objects.as_mut_slice()
    }

    /// Changes camera used by this scene.
    pub fn change_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    /// Returns a reference to camera used by this scene.
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Returns a mutable reference to camera used by this scene.
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn light_color_vec3(&self) -> Vec3 {
        self.light_color
    }

    pub fn light_color(&self) -> (f32, f32, f32) {
        (self.light_color.x, self.light_color.y, self.light_color.z)
    }

    pub fn set_light_color(&mut self, r: f32, g: f32, b: f32) {
        self.light_color = Vec3::new(r, g, b);
    }

    pub fn set_light_position(&mut self, x: f32, y: f32, z: f32) {
        self.light_position = Vec3::new(x, y, z);
    }

    pub fn light_position(&self) -> (f32, f32, f32) {
        (self.light_position.x, self.light_position.y, self.light_position.z)
    }

    pub fn light_data(&self) -> LightData {
        LightData {
            _dummy0: [0; 4],
            light_position: self.light_position.into(),
            light_color: self.light_color.into(),
        }
    }
}