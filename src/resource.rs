use crate::resource::mesh::Vertex;
use crate::renderer::queues::Queues;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use crate::settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;
use crate::resource::mesh::Mesh;
use crate::resource::camera::Camera;
use crate::resource::scene::Scene;

pub mod mesh;
pub mod camera;
pub mod scene;
pub mod object;

pub struct AssetManager<'a> {
    settings: Rc<RefCell<Settings>>,
    active_scene: Option<&'a mut Scene<'a>>,

    scenes: HashMap<String, Scene<'a>>,
    meshes: HashMap<String, Mesh>,

    queues: Queues,
}

impl<'a> AssetManager<'a> {
    pub fn new(settings: Rc<RefCell<Settings>>, queues: Queues) -> Self {
        AssetManager {
            settings,
            active_scene: None,
            scenes: HashMap::new(),
            meshes: HashMap::new(),
            queues,
        }
    }

    pub fn create_mesh<S: Into<String>>(&self, name: S, vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
        Mesh::new(name, vertices, indices, self.queues.graphics_queue())
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.insert(mesh.name().to_string(), mesh);
    }

    pub fn mesh(&self, name: &str) -> Option<&Mesh> {
        self.meshes.get(name)
    }

    pub fn remove_mesh(&mut self, name: &str) -> Option<Mesh> {
        self.meshes.remove(name)
    }

    pub fn add_scene(&mut self, scene: Scene<'a>) {
        self.scenes.insert(scene.name().to_string(), scene);
    }

    pub fn scene_mut(&mut self, name: &str) -> Option<&mut Scene<'a>> {
        self.scenes.get_mut(name)
    }

    pub fn remove_scene(&mut self, name: &str) -> Option<Scene<'a>> {
        self.scenes.remove(name)
    }

    pub fn active_scene(&self) -> Option<&Scene<'a>> {
        match &self.active_scene {
            Some(scene) => Some(&*scene),
            None => None,
        }
    }

    pub fn active_scene_mut(&mut self) -> Option<&mut Scene<'a>> {
        match &mut self.active_scene {
            Some(scene) => Some(scene),
            None => None,
        }
    }
}