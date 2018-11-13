use gfx_hal::{Backend, Adapter, MemoryType, Limits};
use gfx_hal::adapter::PhysicalDevice;

use log::*;

pub struct AdapterState<B: Backend> {
    pub adapter: Option<Adapter<B>>,
    pub memory_types: Vec<MemoryType>,
    pub limits: Limits,
}

impl<B: Backend> AdapterState<B> {
    pub fn new(adapters: &mut Vec<Adapter<B>>) -> Self {
        info!("Chosen: ");

        for adapter in adapters.iter() {
            info!("{:?}", adapter.info);
        }

        AdapterState::<B>::new_adapter(adapters.remove(0))
    }

    pub fn new_adapter(adapter: Adapter<B>) -> Self {
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