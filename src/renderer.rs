mod device_state;
mod window_state;
mod adapter_state;
mod backend_state;
mod desc_set_layout;
mod desc_set;

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
use gfx_hal::{pso, Instance, Adapter, MemoryType, Limits, QueueGroup};
use gfx_hal::format::{AsFormat, Rgba8Srgb};

use gfx_hal::pso::ShaderStageFlags;

use std::cell::RefCell;
use std::rc::Rc;

use log::*;

use crate::renderer::device_state::DeviceState;
use crate::renderer::window_state::WindowState;
use crate::renderer::adapter_state::AdapterState;
use crate::renderer::backend_state::BackendState;
use crate::renderer::desc_set_layout::DescSetLayout;

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

        let image_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::SampledImage,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ],
        );

        let uniform_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            }],
        );
    }

}