use crate::resource::object::Object;
use crate::resource::camera::Camera;

/// Scene is a collection of game objects and world properties.
/// Only one can be active at a time.
#[derive(Clone)]
pub struct Scene {
    name: String,

    camera: Camera,
    objects: Vec<Object>,
}

impl Scene {
    /// Creates new scene.
    pub fn new<S: Into<String>>(name: S, camera: Camera) -> Self {
        Scene {
            name: name.into(),
            camera,
            objects: Vec::new(),
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
}