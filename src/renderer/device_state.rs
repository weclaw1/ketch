use gfx_hal::Backend;
use gfx_hal::{Adapter, QueueGroup};
use gfx_hal::window::Surface;

pub struct DeviceState<B: Backend> {
    pub device: B::Device,
    pub physical_device: B::PhysicalDevice,
    pub queues: QueueGroup<B, gfx_hal::Graphics>,
}

impl<B: Backend> DeviceState<B> {
    pub fn new(mut adapter: Adapter<B>, surface: &B::Surface) -> Self {
        let (device, queues) = adapter
            .open_with::<_, gfx_hal::Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        DeviceState { device, queues, physical_device: adapter.physical_device }
    }
}