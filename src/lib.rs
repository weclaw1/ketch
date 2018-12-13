mod renderer;
pub mod settings;
pub mod resource;
pub mod input;

use crate::input::NoInputMapping;
use crate::resource::AssetManager;
use crate::renderer::{Renderer};
use crate::settings::Settings;
use crate::input::{InputSystem, InputMapping};

use std::cell::RefCell;
use std::rc::Rc;

use std::time::{Duration, Instant};

use log::*;

/// A struct representing the top level of this engine.
/// It provides access to all the subsystems that can be used.
pub struct Smml<T: InputMapping = NoInputMapping> {
    renderer: Renderer,
    asset_manager: AssetManager,
    input_system: InputSystem<T>,
    settings: Rc<RefCell<Settings>>,
}

impl<T: InputMapping> Smml<T> {
    /// Creates and returns a new instance of this engine.
    pub fn new() -> Self {
        let settings = Rc::new(RefCell::new(Settings::new("smml", 800.0, 600.0)));
        let input_system = InputSystem::new(settings.clone());
        let renderer = Renderer::new(settings.clone(), input_system.events_loop()).unwrap();
        let asset_manager = AssetManager::new(settings.clone(), renderer.get_queues());
        
        Smml {
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

    /// Returns reference to input system, which updates input mapping implemented by the user.
    pub fn input_system_mut(&mut self) -> &mut InputSystem<T> {
        &mut self.input_system
    }

    pub fn run<F: FnMut(&mut Settings, &mut AssetManager, &mut InputSystem<T>, Duration)>(&mut self, mut update: F) {
        let time_per_update = self.settings.borrow().time_per_update();

        let mut previous_time = Instant::now();
        let mut lag = Duration::new(0, 0);

        loop {
            let elapsed = previous_time.elapsed();
            previous_time = Instant::now();
            lag += elapsed;

            self.input_system.update();

            while lag >= time_per_update {
                update(&mut self.settings.borrow_mut(), &mut self.asset_manager, &mut self.input_system, time_per_update);

                lag -= time_per_update;
            }

            self.renderer.render(&mut self.asset_manager);
        }
    }

    pub fn asset_manager_mut(&mut self) -> &mut AssetManager {
        &mut self.asset_manager
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
