#[cfg(feature = "dx12")]
use gfx_backend_dx12 as backend;
#[cfg(feature = "gl")]
use gfx_backend_gl as backend;
#[cfg(feature = "metal")]
use gfx_backend_metal as backend;
//#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as backend;

#[cfg(feature = "gl")]
use backend::glutin::GlContext;

use winit::{EventsLoop, WindowBuilder};

use gfx_hal::Instance;

use crate::settings::Settings;


struct WindowState {
    events_loop: EventsLoop,
    window_builder: WindowBuilder,
}

impl WindowState {
    fn new(settings: &Settings) -> WindowState {
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

pub struct Renderer {

}

impl Renderer {

    //#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
    fn create_backend(window_state: &mut WindowState) -> (BackendState<backend::Backend>, backend::Instance) {
        let window = window_state.window_builder
                                 .build(&window_state.events_loop)
                                 .unwrap();

        let instance = backend::Instance::create("smml", 1);
        let surface = instance.create_surface(&window);
        let mut adapters = instance.enumerate_adapters();
        (
            BackendState {
                adapter: AdapterState::new(&mut adapters),
                surface,
                window,
            },
            instance,
        )
    }

    #[cfg(feature = "gl")]
    fn create_backend(window_state: &mut WindowState) -> (BackendState<backend::Backend>, ()) {
        let window = {
            let builder =
                back::config_context(
                    back::glutin::ContextBuilder::new(),
                    ColorFormat::SELF,
                    None,
                )
                .with_vsync(true);
            back::glutin::GlWindow::new(
                window_state.wb.take().unwrap(),
                builder,
                &window_state.events_loop,
            ).unwrap()
        };

        let surface = back::Surface::from_window(window);
        let mut adapters = surface.enumerate_adapters();
        (
            BackendState {
                adapter: AdapterState::new(&mut adapters),
                surface,
            },
            (),
        )
    }
}