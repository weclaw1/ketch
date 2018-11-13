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

use gfx_hal::Backend;
use winit::{EventsLoop, WindowBuilder};

use gfx_hal::window::Surface;
use gfx_hal::{Instance, Adapter, MemoryType, Limits, QueueGroup};
use gfx_hal::format::{AsFormat, Rgba8Srgb};
use gfx_hal::adapter::PhysicalDevice;

use std::cell::RefCell;
use std::rc::Rc;

use log::*;

use crate::render::device_state::DeviceState;
use crate::render::window_state::WindowState;

use crate::settings::Settings;

pub struct Renderer<B: Backend> {

}

impl<B: Backend> Renderer<B> {
    pub fn new(settings: &Settings) {
        let mut window = WindowState::new(settings);
        let backend = BackendState::create_backend(&mut window);

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));
    }

}

struct BackendState<B: Backend> {
    surface: B::Surface,
    adapter: AdapterState<B>,
    window: Option<winit::Window>,
    instance: Option<backend::Instance>,
}

impl<B: Backend> BackendState<B> {

    //#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
    fn create_backend(window: &mut WindowState) -> BackendState<backend::Backend> {
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
    fn create_backend(window: &mut WindowState) -> BackendState<backend::Backend> {
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

struct AdapterState<B: Backend> {
    adapter: Option<Adapter<B>>,
    memory_types: Vec<MemoryType>,
    limits: Limits,
}

impl<B: Backend> AdapterState<B> {
    fn new(adapters: &mut Vec<Adapter<B>>) -> Self {
        info!("Chosen: ");

        for adapter in adapters.iter() {
            info!("{:?}", adapter.info);
        }

        AdapterState::<B>::new_adapter(adapters.remove(0))
    }

    fn new_adapter(adapter: Adapter<B>) -> Self {
        let memory_types = adapter.physical_device.memory_properties().memory_types;
        let limits = adapter.physical_device.limits();
        info!("{:?}", limits);

        AdapterState {
            adapter: Some(adapter),
            memory_types,
            limits,
        }
    }
}

