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
    settings: Rc<RefCell<Settings>>,
    events_loop: EventsLoop,
    surface: Option<Arc<Surface<Window>>>,
    grab_cursor: bool,
    hide_cursor: bool,
}

impl InputSystem {
    /// Creates new input system. At first surface is set to None because renderer is created after input system.
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        let events_loop = EventsLoop::new();

        let grab_cursor = settings.borrow().grab_cursor();
        let hide_cursor = settings.borrow().hide_cursor();

        InputSystem {
            settings: settings,
            events_loop: events_loop,
            surface: None,
            grab_cursor,
            hide_cursor,
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

    /// Fetches pending input.
    pub fn fetch_pending_input(&mut self) -> Vec<InputEvent> {
        let events = self.load_events();

        events.into_iter()
              .filter_map(|event| input_event::to_input_event(event))
              .collect()
    }

    fn load_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        let mut settings = self.settings.borrow_mut();

        if let Some(window) = self.window() {
            let setting_grab_cursor = settings.grab_cursor();
            if self.grab_cursor != setting_grab_cursor {
                match window.grab_cursor(setting_grab_cursor) {
                    Ok(_res) => self.grab_cursor = setting_grab_cursor,
                    Err(err) => error!("Error: {}", err),
                }  
            }
        }

        if let Some(window) = self.window() {
            let setting_hide_cursor = settings.hide_cursor();
            if self.hide_cursor != setting_hide_cursor {
                window.hide_cursor(setting_hide_cursor);
                self.hide_cursor = setting_hide_cursor;
            }
        }

        self.events_loop.poll_events(|input_event| {
            match &input_event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => std::process::exit(0),
                    WindowEvent::Resized(logical_size) => {
                        let dpi = settings.dpi();
                        settings.set_window_size(logical_size.to_physical(dpi));
                    },
                    WindowEvent::HiDpiFactorChanged(dpi) => settings.set_dpi(*dpi),
                    _ => events.push(input_event),
                },
                _ => events.push(input_event),
            }
        });

        events
    }
}
