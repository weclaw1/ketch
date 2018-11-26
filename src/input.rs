pub mod input_event;

use crate::settings::Settings;
use crate::input::input_event::InputEvent;

use winit::EventsLoop;
use winit::Event;
use winit::WindowEvent;

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
        self.input_mapping = Some(mapping);
    }

    pub fn input_mapping(&self) -> Option<&T> {
        self.input_mapping.as_ref()
    }

    pub fn take_input_mapping(&mut self) -> Option<T> {
        self.input_mapping.take()
    }

    pub fn fetch_pending_input(&mut self, settings: &mut Settings) -> Vec<InputEvent> {
        let events = self.load_events(settings);

        events.into_iter()
              .filter_map(|event| input_event::to_input_event(event))
              .collect()
    }

    fn load_events(&mut self, settings: &mut Settings) -> Vec<Event> {
        let events = Vec::new();
        self.events_loop.poll_events(|input_event| {
            match input_event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => std::process::exit(0),
                    WindowEvent::Resized(logical_size) => {
                        settings.set_window_size(logical_size);
                    }
                    _ => events.push(input_event),
                },
                _ => events.push(input_event),
            }
        });

        events
    }

    pub fn update(&mut self, settings: &mut Settings) {
        let events = self.load_events(settings);

        if let Some(input_mapping) = self.input_mapping {
            let input_events: Vec<InputEvent> = events.into_iter()
                                                      .filter_map(|event| input_event::to_input_event(event))
                                                      .collect();

            input_mapping.update_input(input_events);
        }
    }
}

pub trait InputMapping {
    fn update_input(&mut self, input: Vec<InputEvent>);
}