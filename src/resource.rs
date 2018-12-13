use std::sync::Arc;
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

pub struct AssetManager {
    settings: Rc<RefCell<Settings>>,
    active_scene: Option<Scene>,

    scenes: HashMap<String, Scene>,
    meshes: HashMap<String, Arc<Mesh>>,

    queues: Queues,
}

impl AssetManager {
    pub fn new(settings: Rc<RefCell<Settings>>, queues: Queues) -> Self {
        AssetManager {
            settings,
            active_scene: None,
            scenes: HashMap::new(),
            meshes: HashMap::new(),
            queues,
        }
    }

    pub fn create_mesh<S: Into<String>>(&self, name: S, vertices: Vec<Vertex>, indices: Vec<u32>) -> Arc<Mesh> {
        Arc::new(Mesh::new(name, vertices, indices, self.queues.graphics_queue()))
    }

    pub fn add_mesh(&mut self, mesh: Arc<Mesh>) {
        self.meshes.insert(mesh.name().to_string(), mesh);
    }

    pub fn mesh(&self, name: &str) -> Option<Arc<Mesh>> {
        match self.meshes.get(name) {
            Some(mesh) => Some(mesh.clone()),
            None => None,
        }
    }

    pub fn remove_mesh(&mut self, name: &str) -> Option<Arc<Mesh>> {
        self.meshes.remove(name)
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.name().to_string(), scene);
    }

    pub fn scene_mut(&mut self, name: &str) -> Option<&mut Scene> {
        self.scenes.get_mut(name)
    }

    pub fn remove_scene(&mut self, name: &str) -> Option<Scene> {
        self.scenes.remove(name)
    }

    pub fn set_active_scene(&mut self, scene: Scene) {
        self.active_scene = Some(scene);
    }

    pub fn remove_active_scene(&mut self) -> Option<Scene> {
        self.active_scene.take()
    }

    pub fn active_scene(&self) -> Option<&Scene> {
        self.active_scene.as_ref()
    }

    pub fn active_scene_mut(&mut self) -> Option<&mut Scene> {
        self.active_scene.as_mut()
    }
}