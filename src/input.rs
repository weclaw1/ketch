pub mod input_event;

use winit::Window;
use vulkano::swapchain::Surface;
use std::sync::Arc;
use crate::settings::Settings;
use crate::input::input_event::InputEvent;

use std::cell::RefCell;
use std::rc::Rc;

use winit::EventsLoop;
use winit::Event;
use winit::WindowEvent;

/// Manages input. Gives access to input events and updates input mapping defined by the user.
pub struct InputSystem<T: InputMapping = NoInputMapping> {
    settings: Rc<RefCell<Settings>>,
    events_loop: EventsLoop,
    input_mapping: Option<T>,
    surface: Option<Arc<Surface<Window>>>,
}

impl<T: InputMapping> InputSystem<T> {
    /// Creates new input system. At first surface is set to None because renderer is created after input system.
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        let events_loop = EventsLoop::new();

        InputSystem {
            settings: settings,
            events_loop: events_loop,
            input_mapping: None,
            surface: None,
        }
    }

    /// Returns a reference to the events loop.
    pub fn events_loop(&self) -> &EventsLoop {
        &self.events_loop
    }

    /// Sets the input mapping.
    pub fn set_input_mapping(&mut self, mapping: T) {
        self.input_mapping = Some(mapping);
    }

    /// Returns an Option with reference to current input mapping.
    pub fn input_mapping(&self) -> Option<&T> {
        self.input_mapping.as_ref()
    }

    /// Returns an Option with current input mapping. After using this function current input mapping
    /// will be set to None.
    pub fn take_input_mapping(&mut self) -> Option<T> {
        self.input_mapping.take()
    }

    /// Sets the current surface.
    pub fn set_surface(&mut self, surface: Arc<Surface<Window>>) {
        self.surface = Some(surface);
    }

    /// Returns an Option with a reference to the application window.
    pub fn window(&self) -> Option<&Window> {
        self.surface.as_ref().map(|x| x.window())
    }

    /// Fetches pending input. Can be used to implement rebinding keys at runtime.
    pub fn fetch_pending_input(&mut self) -> Vec<InputEvent> {
        let events = self.load_events();

        events.into_iter()
              .filter_map(|event| input_event::to_input_event(event))
              .collect()
    }

    fn load_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        let settings = self.settings.clone();
        self.events_loop.poll_events(|input_event| {
            match &input_event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => std::process::exit(0),
                    WindowEvent::Resized(logical_size) => {
                        let dpi = settings.borrow().dpi();
                        settings.borrow_mut().set_window_size(logical_size.to_physical(dpi));
                    },
                    WindowEvent::HiDpiFactorChanged(dpi) => settings.borrow_mut().set_dpi(*dpi),
                    _ => events.push(input_event),
                },
                _ => events.push(input_event),
            }
        });

        events
    }

    /// Method executed by the engine to update input mapping with pending input events.
    pub fn update(&mut self) {
        let events = self.load_events();

        if let Some(input_mapping) = &mut self.input_mapping {
            let input_events: Vec<InputEvent> = events.into_iter()
                                                      .filter_map(|event| input_event::to_input_event(event))
                                                      .collect();

            input_mapping.update_input(&input_events);
        }
    }
}

pub trait InputMapping {
    fn update_input(&mut self, input: &[InputEvent]);
}

pub struct NoInputMapping {}

impl InputMapping for NoInputMapping {
    fn update_input(&mut self, _input: &[InputEvent]) {

    }
}