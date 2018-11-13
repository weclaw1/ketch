use winit::{EventsLoop, WindowBuilder};
use crate::settings::Settings;

pub struct WindowState {
    pub events_loop: EventsLoop,
    pub window_builder: WindowBuilder,
}

impl WindowState {
    pub fn new(settings: &Settings) -> Self {
        let events_loop = winit::EventsLoop::new();

        let window_builder = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::new(settings.scr_width(), settings.scr_height()))
            .with_title("smml".to_string());

        WindowState {
            events_loop: events_loop,
            window_builder: window_builder,
        }
    }
}