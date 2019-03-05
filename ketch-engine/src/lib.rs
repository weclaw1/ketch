use winit::KeyboardInput;
use ketch_editor::Editor;
use std::error::Error;
use ketch_core::input::input_event::InputEvent;
use ketch_core::resource::AssetManager;
use ketch_core::renderer::{Renderer};
use ketch_core::settings::Settings;
use ketch_core::input::InputSystem;
use ketch_core::input;

use winit::Event;
use winit::WindowEvent;
use winit::VirtualKeyCode;
use winit::ElementState;

pub use ketch_core::renderer::{get_window_dimensions, get_window_dpi};

use std::time::{Duration, Instant};

use fps_counter::FPSCounter;

use structopt::StructOpt;

use log::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opts {
    /// Activate GUI Editor
    #[structopt(short = "g", long = "gui-editor")]
    gui_editor: bool,
}

/// A struct representing the top level of this engine.
/// It provides access to all the subsystems that can be used.
pub struct Engine {
    renderer: Renderer,
    asset_manager: AssetManager,
    input_system: InputSystem,
    editor: Option<Editor>,
    settings: Settings,
}

impl Engine {
    /// Creates and returns a new instance of this engine.
    pub fn new(settings: Settings) -> Self {
        let opts = Opts::from_args();

        let mut input_system = InputSystem::new();
        let renderer = match Renderer::new(&settings, input_system.events_loop()) {
            Ok(renderer) => renderer,
            Err(e) => {
                error!("Couldn't create renderer: {}", e);
                panic!("Couldn't create renderer!");
            },
        };
        input_system.set_surface(renderer.surface());
        let asset_manager = AssetManager::new(renderer.queues(), renderer.device());

        let editor = if opts.gui_editor {
            match Editor::new(&renderer) {
                Ok(editor) => Some(editor),
                Err(e) => {
                    error!("Couldn't create editor: {}", e);
                    panic!("Couldn't create editor!");
                },
            }
        } else {
            None
        };
        
        Engine {
            renderer,
            asset_manager,
            input_system,
            settings,
            editor,
        }
    }

    /// Returns settings used by this engine.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Returns a reference to input system, which updates input mapping implemented by the user.
    pub fn input_system_mut(&mut self) -> &mut InputSystem {
        &mut self.input_system
    }

    /// Returns a mutable reference to the asset manager.
    pub fn asset_manager_mut(&mut self) -> &mut AssetManager {
        &mut self.asset_manager
    }

    fn handle_input<S: EventHandler>(&mut self, game: &mut S) {
        let pending_events = self.input_system.fetch_pending_events();
        let mut esc_pressed = false;

        for event in pending_events.iter() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => std::process::exit(0),
                    WindowEvent::Resized(_window_size) => self.renderer.force_recreate_swapchain(),
                    WindowEvent::HiDpiFactorChanged(_dpi) => self.renderer.force_recreate_swapchain(),
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            .. 
                        } => esc_pressed = true,
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        }

        match &mut self.editor {
            Some(editor) => {
                if editor.run_game() && esc_pressed {
                    self.input_system.grab_cursor(false);
                    self.input_system.hide_cursor(false);
                    editor.set_run_game(false, Some(&self.asset_manager));
                } else if editor.run_game() && !esc_pressed {
                    game.process_input(&mut self.input_system, input::convert_to_input_events(pending_events));
                } else {
                    editor.handle_input(pending_events, &mut self.input_system);
                }
            },
            None => game.process_input(&mut self.input_system, input::convert_to_input_events(pending_events)),
        }  
    }

    fn update<S: EventHandler>(&mut self, game: &mut S, time_per_update: Duration) {
        match &mut self.editor {
            Some(editor) => {
                if editor.run_game() {
                    game.update(&self.settings, &mut self.asset_manager, time_per_update);
                } else {
                    editor.update(&mut self.asset_manager, time_per_update);
                }
            },
            None => game.update(&self.settings, &mut self.asset_manager, time_per_update),
        }
    }

    pub fn run<S: EventHandler>(&mut self, mut game: S) {
        let mut fps_counter = FPSCounter::new();
        let log_fps_frequency = self.settings.log_fps_frequency();
        let time_per_update = self.settings.time_per_update();

        let mut last_fps_counter_log = Instant::now();
        let mut previous_time = Instant::now();
        let mut lag = Duration::new(0, 0);

        game.init(&self.settings, &mut self.asset_manager);

        if let Some(editor) = &mut self.editor {
            editor.sync_editor(&mut self.asset_manager);
        }

        loop {
            let elapsed = previous_time.elapsed();
            previous_time = Instant::now();
            lag += elapsed;
            
            self.handle_input(&mut game);

            while lag >= time_per_update {
                self.update(&mut game, time_per_update);

                lag -= time_per_update;
            }

            let mut command_buffer = match self.renderer.create_command_buffer() {
                Ok(res) => res,
                Err(err) => {
                    error!("Couldn't create command buffer: {}", err);
                    continue;
                }
            };

            if let Some(editor) = &mut self.editor {
                if !editor.run_game() {
                    command_buffer = editor.add_glyph_commands(command_buffer);
                }
            }

            let (image_num, acquire_future, mut command_buffer) = match self.renderer.render_scene(command_buffer, &mut self.asset_manager) {
                Ok(res) => res,
                Err(err) => {
                    error!("Couldn't render scene: {}", err);
                    continue;
                }
            };

            if let Some(editor) = &mut self.editor {
                if !editor.run_game() {
                    command_buffer = editor.add_draw_commands(self.renderer.queues().graphics_queue(), command_buffer);
                }
            }

            match self.renderer.execute_command_buffer(image_num, acquire_future, command_buffer) {
                Ok(()) => {
                    let fps = fps_counter.tick();
                    if last_fps_counter_log.elapsed() >= log_fps_frequency {
                        info!("Current FPS: {}", fps);
                        last_fps_counter_log = Instant::now();
                    }
                },
                Err(err) => {
                    error!("Couldn't execute command buffer for frame: {}", err);
                } 
            }
        }
    }
}

pub trait EventHandler {
    fn process_input(&mut self, input_system: &mut InputSystem, input_events: Vec<InputEvent>);
    fn update(&mut self, settings: &Settings, asset_manager: &mut AssetManager, elapsed_time: Duration);
    fn init(&mut self, settings: &Settings, asset_manager: &mut AssetManager);
}