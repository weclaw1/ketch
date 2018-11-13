#[cfg(feature = "dx12")]
use gfx_backend_dx12 as backend;
#[cfg(feature = "gl")]
use gfx_backend_gl as backend;
#[cfg(feature = "metal")]
use gfx_backend_metal as backend;
//#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as backend;

use gfx_hal::{Backend, Instance};

use crate::renderer::adapter_state::AdapterState;
use crate::renderer::window_state::WindowState;

pub struct BackendState<B: Backend> {
    pub surface: B::Surface,
    pub adapter: AdapterState<B>,
    pub window: Option<winit::Window>,
    pub instance: Option<backend::Instance>,
}

impl<B: Backend> BackendState<B> {

    //#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
    pub fn create_backend(window: &mut WindowState) -> BackendState<backend::Backend> {
        let window = window.window_builder
                                 .build(&window.events_loop)
                                 .unwrap();

        let instance = backend::Instance::create("smml", 1);
        let surface = instance.create_surface(&window);
        let mut adapters = instance.enumerate_adapters();
        BackendState {
            adapter: AdapterState::new(&mut adapters),
            surface: surface,
            window: Some(window),
            instance: Some(instance),
        }
    }

    #[cfg(feature = "gl")]
    pub fn create_backend(window: &mut WindowState) -> BackendState<backend::Backend> {
        let builder = backend::config_context(
                        backend::glutin::ContextBuilder::new(),
                        Rgba8Srgb::SELF,
                        None,
                     ).with_vsync(true);

        let window = backend::glutin::GlWindow::new(
                        window.window_builder,
                        builder,
                        &window.events_loop,
                     ).unwrap();

        let surface = backend::Surface::from_window(window);
        let mut adapters = surface.enumerate_adapters();
        BackendState {
            adapter: AdapterState::new(&mut adapters),
            surface: surface,
            window: None,
            instance: None,
        }
    }
}