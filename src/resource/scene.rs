use crate::resource::object::Object;
use crate::resource::camera::Camera;

#[derive(Clone)]
pub struct Scene<'a> {
    name: String,

    camera: Camera,
    objects: Vec<Object<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new<S: Into<String>>(name: S, camera: Camera) -> Self {
        Scene {
            name: name.into(),
            camera,
            objects: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_object(&mut self, object: Object<'a>) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, id: u32) -> Option<Object<'a>> {
        let object_index = self.objects.iter().position(|x| x.id() == id);
        match object_index {
            Some(index) => Some(self.objects.remove(index)),
            None => None,
        }
    }

    pub fn remove_objects_with_name(&mut self, name: &str) {
        let object_indexes: Vec<usize> = self.objects.iter()
                                                     .enumerate()
                                                     .filter(|(i, x)| x.name() == name)
                                                     .map(|(i, x)| i)
                                                     .collect();

        for index in object_indexes {
            self.objects.remove(index);
        }
    }

    pub fn objects(&self) -> &[Object<'a>] {
        self.objects.as_slice()
    }

    pub fn objects_mut(&mut self) -> &mut [Object<'a>] {
        self.objects.as_mut_slice()
    }

    pub fn change_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}