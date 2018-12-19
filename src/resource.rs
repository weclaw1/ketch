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

/// Manages game assets and scenes.
pub struct AssetManager {
    settings: Rc<RefCell<Settings>>,
    active_scene: Option<Scene>,

    scenes: HashMap<String, Scene>,
    meshes: HashMap<String, Arc<Mesh>>,

    queues: Queues,
}

impl AssetManager {
    /// Creates new asset manager.
    pub fn new(settings: Rc<RefCell<Settings>>, queues: Queues) -> Self {
        AssetManager {
            settings,
            active_scene: None,
            scenes: HashMap::new(),
            meshes: HashMap::new(),
            queues,
        }
    }

    /// Creates a new mesh.
    pub fn create_mesh<S: Into<String>>(&self, name: S, vertices: Vec<Vertex>, indices: Vec<u32>) -> Arc<Mesh> {
        Arc::new(Mesh::new(name, vertices, indices, self.queues.graphics_queue()))
    }

    /// Adds mesh to asset manager. Meshes need to have unique name. 
    /// If two meshes have the same name, the old mesh will be replaced with the new one.
    pub fn add_mesh(&mut self, mesh: Arc<Mesh>) {
        self.meshes.insert(mesh.name().to_string(), mesh);
    }

    /// Returns a mesh with the given name.
    pub fn mesh(&self, name: &str) -> Option<Arc<Mesh>> {
        match self.meshes.get(name) {
            Some(mesh) => Some(mesh.clone()),
            None => None,
        }
    }

    /// Removes and returns a mesh with the given name.
    pub fn remove_mesh(&mut self, name: &str) -> Option<Arc<Mesh>> {
        self.meshes.remove(name)
    }

    /// Adds scene to asset manager. Scenes need to have unique name. 
    /// If two scenes have the same name, the old scene will be replaced with the new one.
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.name().to_string(), scene);
    }

    /// Returns an Option with a mutable reference to the scene with a given name.
    pub fn scene_mut(&mut self, name: &str) -> Option<&mut Scene> {
        self.scenes.get_mut(name)
    }

    /// Removes and returns a scene with given name.
    pub fn remove_scene(&mut self, name: &str) -> Option<Scene> {
        self.scenes.remove(name)
    }

    /// Sets active scene
    pub fn set_active_scene(&mut self, scene: Scene) {
        self.active_scene = Some(scene);
    }

    /// Removes and returns active scene.
    pub fn remove_active_scene(&mut self) -> Option<Scene> {
        self.active_scene.take()
    }

    /// Returns an Option with reference to the active scene.
    pub fn active_scene(&self) -> Option<&Scene> {
        self.active_scene.as_ref()
    }

    /// Returns an Option with mutable reference to the active scene.
    pub fn active_scene_mut(&mut self) -> Option<&mut Scene> {
        self.active_scene.as_mut()
    }
}