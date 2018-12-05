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

const DURATION_PER_UPDATE: Duration = Duration::from_millis(16);


/// A struct representing the top level of this library.
/// It provides access to all the subsystems that can be used.
pub struct Smml<T: InputMapping> {
    renderer: Renderer,
    asset_manager: AssetManager,
    input_system: InputSystem<T>,
    settings: Rc<RefCell<Settings>>,
}

impl<T: InputMapping> Smml<T> {
    pub fn new(settings: Settings) -> Self {
        let settings = Rc::new(RefCell::new(settings));
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

    pub fn run<F: FnMut(&mut Settings, &mut AssetManager, &mut InputSystem<T>, Duration)>(&mut self, mut update: F) {
        let mut previous_time = Instant::now();
        let mut lag = Duration::new(0, 0);

        loop {
            let elapsed = previous_time.elapsed();
            previous_time = Instant::now();
            lag += elapsed;

            self.input_system.update();

            while lag >= DURATION_PER_UPDATE {
                update(&mut self.settings.borrow_mut(), &mut self.asset_manager, &mut self.input_system, DURATION_PER_UPDATE);

                lag -= DURATION_PER_UPDATE;
            }

            self.renderer.render(&mut self.asset_manager);
        }
    }

    pub fn asset_manager(&mut self) -> &mut AssetManager {
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
