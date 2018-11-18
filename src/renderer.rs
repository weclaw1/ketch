mod queues;

use log::*;

use crate::settings::Settings;

use vulkano::instance::{Instance, InstanceCreationError, PhysicalDevice, PhysicalDeviceType, PhysicalDevicesIter};
use vulkano::device::{Device};
use winit::dpi::LogicalSize;
use winit::{EventsLoop, WindowBuilder};

use vulkano_win::{VkSurfaceBuild, CreationError as WindowCreationError};

use crate::renderer::queues::{find_queues, Queues};


pub struct Renderer {

}

impl Renderer {
    pub fn new(settings: &Settings) -> Result<Self, RendererCreationError> {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None)
        }?;

        let physical_device = match rank_devices(PhysicalDevice::enumerate(&instance)) {
            Some(device) => {
                info!("Using device: {} (type: {:?})", device.name(), device.ty());
                device
            },
            None => panic!("Couldn't find physical device!")
        };

        let events_loop = EventsLoop::new();
        let surface = WindowBuilder::new().with_title(settings.window_title())
                                          .with_dimensions(LogicalSize::new(settings.scr_width(), settings.scr_height()))
                                          .build_vk_surface(&events_loop, instance.clone())?;
        let window = surface.window();

        let physical_queues = queues::find_queues(&physical_device, &surface);

        let minimal_features = vulkano::device::Features {
            depth_clamp: true, //needed for correct shadow mapping
            .. vulkano::device::Features::none()
        };

        let device_extensions_needed = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };

        let (device, mut queues) = Device::new(
            physical_device, &minimal_features,
            &device_extensions_needed, physical_queues.iter().cloned()
        ).expect("failed to create device");

        let queues = Queues::new(queues);

        Ok(Renderer {})
    }

}


fn rank_devices(devices: PhysicalDevicesIter) -> Option<PhysicalDevice> {
    devices.into_iter().map(|device|
        match device.ty() {
            PhysicalDeviceType::DiscreteGpu => (device, 4),
            PhysicalDeviceType::VirtualGpu => (device, 3),
            PhysicalDeviceType::IntegratedGpu => (device, 2),
            PhysicalDeviceType::Cpu => (device, 1),
            PhysicalDeviceType::Other => (device, 0),
        }
    ).max_by(|x, y| x.1.cmp(&y.1)).map(|(device, _)| device)
}

#[derive(Debug)]
pub enum RendererCreationError {
    InstanceCreationError(InstanceCreationError),
    WindowCreationError(WindowCreationError),
} 

impl std::fmt::Display for RendererCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RendererCreationError::InstanceCreationError(e) => e.fmt(f),
            RendererCreationError::WindowCreationError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for RendererCreationError {
    fn description(&self) -> &str {
        match self {
            RendererCreationError::InstanceCreationError(e) => e.description(),
            RendererCreationError::WindowCreationError(e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            RendererCreationError::InstanceCreationError(e) => Some(e),
            RendererCreationError::WindowCreationError(e) => Some(e),
        }
    }
}

impl From<InstanceCreationError> for RendererCreationError {
    fn from(err: InstanceCreationError) -> RendererCreationError {
        RendererCreationError::InstanceCreationError(err)
    }
}

impl From<WindowCreationError> for RendererCreationError {
    fn from(err: WindowCreationError) -> RendererCreationError {
        RendererCreationError::WindowCreationError(err)
    }
}