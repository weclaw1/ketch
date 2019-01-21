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

use log::*;

/// Manages input. Fetches input events and manages window.
pub struct InputSystem {
    events_loop: EventsLoop,
    surface: Option<Arc<Surface<Window>>>,
}

impl InputSystem {
    /// Creates new input system. At first surface is set to None because renderer is created after input system.
    pub fn new() -> Self {
        let events_loop = EventsLoop::new();

        InputSystem {
            events_loop: events_loop,
            surface: None,
        }
    }

    /// Returns a reference to the events loop.
    pub fn events_loop(&self) -> &EventsLoop {
        &self.events_loop
    }

    /// Sets the current surface.
    pub fn set_surface(&mut self, surface: Arc<Surface<Window>>) {
        self.surface = Some(surface);
    }

    /// Returns an Option with a reference to the application window.
    pub fn window(&self) -> Option<&Window> {
        self.surface.as_ref().map(|x| x.window())
    }

    /// Grabs cursor, preventing it from leaving the window.
    pub fn grab_cursor(&self, value: bool) {
        if let Some(window) = self.window() {
            if let Err(err) = window.grab_cursor(value) {
                error!("Error: {}", err);
            }
        }
    }

    /// Hides the cursor, making it invisible but still usable.
    pub fn hide_cursor(&mut self, value: bool) {
        if let Some(window) = self.window() {
            window.hide_cursor(value);
        }
    }

    /// Loads pending events
    pub fn fetch_pending_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();

        self.events_loop.poll_events(|input_event| {
            events.push(input_event);
        });

        events
    }
}

/// Converts winit events to InputEvents
pub fn convert_to_input_events(events: Vec<Event>) -> Vec<InputEvent> {
    events.into_iter()
          .filter_map(|event| input_event::to_input_event(event))
          .collect()
}
