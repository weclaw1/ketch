#[cfg(feature = "dx12")]
use gfx_backend_dx12 as backend;
#[cfg(feature = "gl")]
use gfx_backend_gl as backend;
#[cfg(feature = "metal")]
use gfx_backend_metal as backend;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as backend;

#[cfg(feature = "gl")]
use backend::glutin::GlContext;

use winit::{EventsLoop, WindowBuilder};

// settings
const SCR_WIDTH: f64 = 800.0;
const SCR_HEIGHT: f64 = 600.0;

struct WindowState {
    events_loop: EventsLoop,
    wb: WindowBuilder,
}

impl WindowState {
    fn new() -> WindowState {
        let events_loop = winit::EventsLoop::new();

        let wb = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::new(DIMS.width as _, DIMS.height as _))
            .with_title("quad".to_string());

        WindowState {
            events_loop: events_loop,
            wb: wb,
        }
    }
}

pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Self {

    }
}