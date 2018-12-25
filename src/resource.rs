use image::DynamicImage;
use std::sync::RwLock;
use image::RgbaImage;
use std::path::Path;
use vulkano::device::Device;
use crate::resource::texture::Texture;
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
pub mod texture;

const DEFAULT_TEXTURE_NAME: &'static str = "default";

/// Manages game assets and scenes.
pub struct AssetManager {
    settings: Rc<RefCell<Settings>>,
    active_scene: Option<Scene>,

    scenes: HashMap<String, Scene>,
    meshes: HashMap<String, Arc<RwLock<Mesh>>>,
    textures: HashMap<String, Arc<Texture>>,

    device: Arc<Device>,
    queues: Queues,
}

impl AssetManager {
    /// Creates new asset manager.
    pub fn new(settings: Rc<RefCell<Settings>>, queues: Queues, device: Arc<Device>) -> Self {
        let image = image::load_from_memory(include_bytes!("../data/default.png")).unwrap();
        let default_texture = Arc::new(Texture::new(DEFAULT_TEXTURE_NAME, image, queues.graphics_queue(), device.clone()));
        let mut textures = HashMap::new();
        textures.insert(DEFAULT_TEXTURE_NAME.to_string(), default_texture);
        AssetManager {
            settings,
            active_scene: None,
            scenes: HashMap::new(),
            meshes: HashMap::new(),
            textures,
            queues,
            device,
        }
    }

    /// Creates a new mesh.
    pub fn create_mesh<S: Into<String>>(&self, name: S, vertices: Vec<Vertex>, indices: Vec<u32>) -> Arc<RwLock<Mesh>> {
        Arc::new(RwLock::new(Mesh::new(name, vertices, indices, self.textures.get(DEFAULT_TEXTURE_NAME).unwrap().clone(), self.queues.graphics_queue())))
    }

    /// Adds mesh to asset manager. Meshes need to have unique name. 
    /// If two meshes have the same name, the old mesh will be replaced with the new one.
    pub fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        let name = mesh.read().unwrap().name().to_string();
        self.meshes.insert(name, mesh);
    }

    /// Returns a mesh with the given name.
    pub fn mesh(&self, name: &str) -> Option<Arc<RwLock<Mesh>>> {
        match self.meshes.get(name) {
            Some(mesh) => Some(mesh.clone()),
            None => None,
        }
    }

    /// Removes and returns a mesh with the given name.
    pub fn remove_mesh(&mut self, name: &str) -> Option<Arc<RwLock<Mesh>>> {
        self.meshes.remove(name)
    }

    /// Loads and creates texture from file.
    pub fn load_texture<S: Into<String>, P: AsRef<Path>>(&self, name: S, image_path: P) -> Arc<Texture> {
        Arc::new(Texture::load(name, image_path, self.queues.graphics_queue(), self.device.clone()))
    }

    /// Creates texture from loaded image.
    pub fn create_texture<S: Into<String>>(&self, name: S, image: DynamicImage) -> Arc<Texture> {
        Arc::new(Texture::new(name, image, self.queues.graphics_queue(), self.device.clone()))
    }

    /// Adds texture to asset manager. Textures need to have unique name. 
    /// If two textures have the same name, the old texture will be replaced with the new one.
    pub fn add_texture(&mut self, texture: Arc<Texture>) {
        self.textures.insert(texture.name().to_string(), texture);
    }

    /// Returns a texture with the given name.
    pub fn texture(&self, name: &str) -> Option<Arc<Texture>> {
        match self.textures.get(name) {
            Some(texture) => Some(texture.clone()),
            None => None,
        }
    }

    /// Removes and returns a texture with the given name.
    pub fn remove_texture(&mut self, name: &str) -> Option<Arc<Texture>> {
        if name != DEFAULT_TEXTURE_NAME {
            self.textures.remove(name)
        } else {
            None
        }
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