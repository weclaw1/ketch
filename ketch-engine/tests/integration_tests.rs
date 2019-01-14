use ketch_engine::Engine;
use ketch_core::settings::Settings;
use ketch_core::input::InputSystem;
use ketch_core::renderer::Renderer;
use ketch_core::resource::AssetManager;
use ketch_core::resource::camera::Camera;
use ketch_core::resource::scene::Scene;
use ketch_core::resource::object::ObjectBuilder;

mod common;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;

#[test]
#[ignore]
fn engine_is_created() {
    let settings = Settings::new("test", 600.0, 400.0);
    let _engine = Engine::new(settings);
}

#[test]
#[ignore]
fn surface_is_set_in_input_after_engine_creation() {
    let settings = Settings::new("test", 600.0, 400.0);
    let mut engine = Engine::new(settings);

    assert!(engine.input_system_mut().window().is_some())
}

#[test]
#[ignore]
fn fetch_pending_input_does_not_panic() {
    let settings = Settings::new("test", 600.0, 400.0);
    let mut engine = Engine::new(settings);

    let _input = engine.input_system_mut().fetch_pending_input();
}

#[test]
#[ignore]
fn render_renders_empty_frame_without_error() {
    let settings = Rc::new(RefCell::new(Settings::new("test", 600.0, 400.0)));
    let input_system = InputSystem::new(settings.clone());
    
    let mut renderer = Renderer::new(settings.clone(), input_system.events_loop()).unwrap();
    let mut asset_manager = AssetManager::new(settings.clone(), renderer.queues(), renderer.device());

    assert!(renderer.render(&mut asset_manager).is_ok())
}

#[test]
#[ignore]
fn render_simple_cube_without_texture() {
    let settings = Rc::new(RefCell::new(Settings::new("test", 600.0, 400.0)));
    let input_system = InputSystem::new(settings.clone());
    
    let mut renderer = Renderer::new(settings.clone(), input_system.events_loop()).unwrap();
    let mut asset_manager = AssetManager::new(settings.clone(), renderer.queues(), renderer.device());

    let mesh = asset_manager.create_mesh("test_mesh", common::model::generate_vertices(), common::model::generate_indices());
    asset_manager.add_mesh(mesh);
    let camera = Camera::new(settings.clone());
    asset_manager.set_active_scene(Scene::new("test_scene", camera));
    let object = ObjectBuilder::new("test_object").with_mesh(asset_manager.mesh("test_mesh").unwrap()).build();
    asset_manager.active_scene_mut().unwrap().add_object(object);

    assert!(renderer.render(&mut asset_manager).is_ok())
}

#[test]
#[ignore]
fn render_simple_cube_with_texture() {
    let settings = Rc::new(RefCell::new(Settings::new("test", 600.0, 400.0)));
    let input_system = InputSystem::new(settings.clone());
    
    let mut renderer = Renderer::new(settings.clone(), input_system.events_loop()).unwrap();
    let mut asset_manager = AssetManager::new(settings.clone(), renderer.queues(), renderer.device());

    let mesh = asset_manager.create_mesh("test_mesh", common::model::generate_vertices(), common::model::generate_indices());
    let texture = asset_manager.load_texture("test_texture", Path::new("tests/common/data/rust_logo.png"));
    asset_manager.add_texture(texture.clone());
    mesh.write().unwrap().set_texture(texture);
    asset_manager.add_mesh(mesh);
    let camera = Camera::new(settings.clone());
    asset_manager.set_active_scene(Scene::new("test_scene", camera));
    let object = ObjectBuilder::new("test_object").with_mesh(asset_manager.mesh("test_mesh").unwrap()).build();
    asset_manager.active_scene_mut().unwrap().add_object(object);

    assert!(renderer.render(&mut asset_manager).is_ok())
}

