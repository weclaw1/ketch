mod model;

use std::time::Duration;
use ketch_core::resource::AssetManager;
use ketch_core::settings::Settings;
use ketch_engine::EventHandler;
use ketch_core::input::input_event::ElementState::Released;
use ketch_core::input::input_event::ElementState::Pressed;
use std::path::Path;

use ketch_core::input::input_event::{InputEvent, KeyboardInput, VirtualKeyCode};
use ketch_core::input::InputSystem;
use ketch_core::resource::camera::Direction;
use ketch_engine::Engine;
use ketch_core::resource::scene::Scene;
use ketch_core::resource::camera::Camera;
use ketch_core::resource::object::ObjectBuilder;

pub struct GameInput {
    mouse_delta_changed: bool,
    camera_speed: f32,
    mouse_sensitivity: f32,
    mouse_delta: (f32, f32),
    grab_and_hide_cursor: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl GameInput {
    pub fn new(camera_speed: f32, mouse_sensitivity: f32) -> Self {
        GameInput {
            mouse_delta_changed: false,
            camera_speed,
            mouse_sensitivity,
            mouse_delta: (0.0, 0.0),
            grab_and_hide_cursor: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn up(&self) -> bool {
        self.up
    }

    pub fn down(&self) -> bool {
        self.down
    }

    pub fn left(&self) -> bool {
        self.left
    }

    pub fn right(&self) -> bool {
        self.right
    }

    pub fn grab_and_hide_cursor(&self) -> bool {
        self.grab_and_hide_cursor
    }

    pub fn update_input(&mut self, input_system: &mut InputSystem, input: Vec<InputEvent>) {
        input.into_iter().for_each(|event| match event {
            InputEvent::KeyboardInput { keycode, state } => match keycode {
                VirtualKeyCode::W if state == Pressed => self.up = true,
                VirtualKeyCode::S if state == Pressed => self.down = true,
                VirtualKeyCode::A if state == Pressed => self.left = true,
                VirtualKeyCode::D if state == Pressed => self.right = true,
                VirtualKeyCode::W if state == Released => self.up = false,
                VirtualKeyCode::S if state == Released => self.down = false,
                VirtualKeyCode::A if state == Released => self.left = false,
                VirtualKeyCode::D if state == Released => self.right = false,
                VirtualKeyCode::G if state == Pressed => {
                    input_system.grab_cursor(true);
                    input_system.hide_cursor(true);
                },
                VirtualKeyCode::H if state == Pressed => {
                    input_system.grab_cursor(false);
                    input_system.hide_cursor(false);
                },
                _ => (),
            },
            InputEvent::MouseMotion { delta } => {
                self.mouse_delta_changed = true;
                self.mouse_delta = (delta.0 as f32, delta.1 as f32);
            },
            _ => (),
        })
    }

    pub fn update_camera(&mut self, camera: &mut Camera, elapsed_time: Duration) {
        if self.mouse_delta_changed {
            let (x_delta, y_delta) = self.mouse_delta;

            let current_yaw = camera.yaw();
            camera.set_yaw(current_yaw + x_delta * self.mouse_sensitivity);

            let current_pitch = camera.pitch();
            camera.set_pitch(current_pitch + (-y_delta) * self.mouse_sensitivity);
            self.mouse_delta_changed = false;
        }
        if self.up {
            camera.move_camera(Direction::Up, self.camera_speed * (elapsed_time.as_millis() as f32 / 1000.0));
        }
        if self.down {
            camera.move_camera(Direction::Down, self.camera_speed * (elapsed_time.as_millis() as f32 / 1000.0));
        }
        if self.left {
            camera.move_camera(Direction::Left, self.camera_speed * (elapsed_time.as_millis() as f32 / 1000.0));
        }
        if self.right {
            camera.move_camera(Direction::Right, self.camera_speed * (elapsed_time.as_millis() as f32 / 1000.0));
        }
    }

}

pub struct GameState {
    input: GameInput,
    cubes_positions: Vec<(f32, f32, f32)>,
}

impl GameState {
    pub fn new(camera_speed: f32, mouse_sensitivity: f32) -> Self {
        GameState {
            input: GameInput::new(camera_speed, mouse_sensitivity), 
            cubes_positions: vec!((1.0, 0.0, 0.0), (3.0, -1.0, 2.0), (5.0, 2.0, 4.0)),
        }
    }
}

impl EventHandler for GameState {
    fn init(&mut self, settings: &Settings, asset_manager: &mut AssetManager) {
        let mesh = asset_manager.create_mesh("test_mesh", model::generate_vertices(), model::generate_indices());
        let texture = asset_manager.load_texture("crate", Path::new("example/data/crate.jpg"));
        asset_manager.add_texture(texture.clone());
        mesh.write().unwrap().set_texture(texture);
        asset_manager.add_mesh(mesh);
        let camera = Camera::new();
        asset_manager.set_active_scene(Scene::new("test_scene", camera));
        //asset_manager.active_scene_mut().unwrap().set_light_color(1.0, 0.0, 0.0);
        let mut object_builder = ObjectBuilder::new("test_object").with_mesh(asset_manager.mesh("test_mesh").unwrap());
        for (x, y, z) in self.cubes_positions.iter() {
            object_builder = object_builder.with_position(*x, *y, *z);
            let object = object_builder.build();
            asset_manager.active_scene_mut().unwrap().add_object(object);
        }
        let mut light_obj = ObjectBuilder::new("light_object").with_scale(0.2, 0.2, 0.2).with_position(-1.0, 1.0, 0.0).with_mesh(asset_manager.mesh("test_mesh").unwrap()).build();
        light_obj.set_light_source(true);
        asset_manager.active_scene_mut().unwrap().add_object(light_obj);
        asset_manager.active_scene_mut().unwrap().set_light_position(-1.0, 1.0, 0.0);
    }
    fn process_input(&mut self, input_system: &mut InputSystem, input_events: Vec<InputEvent>) {
        self.input.update_input(input_system, input_events);
    }
    fn update(&mut self, settings: &Settings, asset_manager: &mut AssetManager, elapsed_time: Duration) {
        for object in asset_manager.active_scene_mut().unwrap().objects_mut().iter_mut().filter(|x| x.name() == "test_object") {
            let (x, y, z) = object.rotation_angles();
            object.set_rotation_angles(x, y + 0.01, z);
        }
        self.input.update_camera(asset_manager.active_scene_mut().unwrap().camera_mut(), elapsed_time);
    }
}

fn main() {
    env_logger::init();
    let mut engine = Engine::new(Settings::new("ŚWIATEŁA", 1024.0, 768.0));
    let time_per_update = engine.settings().time_per_update();
    let state = GameState::new(5.0, 0.2);

    engine.run(state);
}
