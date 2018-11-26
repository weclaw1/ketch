use crate::settings::Settings;

use winit::EventsLoop;
use winit::Event;

pub struct InputSystem<T: InputMapping> {
    events_loop: EventsLoop,
    input_mapping: Option<T>,
}


impl<T: InputMapping> InputSystem<T> {
    pub fn new() -> Self {
        let events_loop = EventsLoop::new();

        InputSystem {
            events_loop: events_loop,
            input_mapping: None,
        }
    }

    pub fn events_loop(&self) -> &EventsLoop {
        &self.events_loop
    }

    pub fn set_input_mapping(&mut self, mapping: T) {
        self.input_mapping = Some(T);
    }

    pub fn input_mapping(&self) -> Option<&T> {
        self.input_mapping.as_ref()
    }

    pub fn update(&mut self, settings: &mut Settings) {

    }
}

pub trait InputMapping {
    fn update_input(&mut self, input: Vec<Event>);
}