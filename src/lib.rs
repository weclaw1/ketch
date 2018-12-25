mod renderer;
pub mod settings;
pub mod resource;
pub mod input;

use std::error::Error;
use crate::input::input_event::InputEvent;
use crate::resource::AssetManager;
use crate::renderer::{Renderer};
use crate::settings::Settings;
use crate::input::InputSystem;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use fps_counter::FPSCounter;

use log::*;

/// A struct representing the top level of this engine.
/// It provides access to all the subsystems that can be used.
pub struct Engine {
    renderer: Renderer,
    asset_manager: AssetManager,
    input_system: InputSystem,
    settings: Rc<RefCell<Settings>>,
}

impl Engine {
    /// Creates and returns a new instance of this engine.
    pub fn new(settings: Settings) -> Self {
        let settings = Rc::new(RefCell::new(settings));
        let mut input_system = InputSystem::new(settings.clone());
        let renderer = match Renderer::new(settings.clone(), input_system.events_loop()) {
            Ok(renderer) => renderer,
            Err(e) => {
                error!("Error: {}", e);
                error!("Caused by: {}", e.cause().unwrap());
                panic!("Couldn't create renderer!");
            },
        };
        input_system.set_surface(renderer.surface());
        let asset_manager = AssetManager::new(settings.clone(), renderer.queues(), renderer.device());
        
        Engine {
            renderer,
            asset_manager,
            input_system,
            settings,
        }
    }

    /// Returns settings used by this engine.
    pub fn settings(&self) -> Rc<RefCell<Settings>> {
        self.settings.clone()
    }

    /// Returns a reference to input system, which updates input mapping implemented by the user.
    pub fn input_system_mut(&mut self) -> &mut InputSystem {
        &mut self.input_system
    }

    /// Returns a mutable reference to the asset manager.
    pub fn asset_manager_mut(&mut self) -> &mut AssetManager {
        &mut self.asset_manager
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn run<S: EventHandler>(&mut self, mut state: S) {
        let mut fps_counter = FPSCounter::new();
        let log_fps_frequency = self.settings.borrow().log_fps_frequency();
        let time_per_update = self.settings.borrow().time_per_update();

        let mut last_fps_counter_log = Instant::now();
        let mut previous_time = Instant::now();
        let mut lag = Duration::new(0, 0);

        state.init(self.settings.clone(), &mut self.asset_manager);

        loop {
            let elapsed = previous_time.elapsed();
            previous_time = Instant::now();
            lag += elapsed;

            state.process_input(self.input_system.fetch_pending_input());

            while lag >= time_per_update {
                state.update(&mut self.settings.borrow_mut(), &mut self.asset_manager, time_per_update);

                lag -= time_per_update;
            }

            match self.renderer.render(&mut self.asset_manager) {
                Ok(()) => {
                    let fps = fps_counter.tick();
                    if last_fps_counter_log.elapsed() >= log_fps_frequency {
                        info!("Current FPS: {}", fps);
                        last_fps_counter_log = Instant::now();
                    }
                },
                Err(err) => error!("Couldn't render frame: {}", err),
            }
        }
    }
}

pub trait EventHandler {
    fn process_input(&mut self, input_events: Vec<InputEvent>);
    fn update(&mut self, settings: &mut Settings, asset_manager: &mut AssetManager, elapsed_time: Duration);
    fn init(&mut self, settings: Rc<RefCell<Settings>>, asset_manager: &mut AssetManager);
}