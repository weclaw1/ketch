pub mod queues;
mod uniform_manager;
pub mod shader;

use crate::resource::AssetManager;
use std::cell::RefCell;
use std::rc::Rc;
use log::*;

use crate::settings::Settings;

use vulkano::instance::{Instance, InstanceCreationError, PhysicalDevice, PhysicalDeviceType, PhysicalDevicesIter};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::viewport::Viewport;
use vulkano::image::SwapchainImage;
use vulkano::swapchain::{Surface, PresentMode, Swapchain, SurfaceTransform, CompositeAlpha};
use vulkano::single_pass_renderpass;
use vulkano::framebuffer::{RenderPassAbstract, Framebuffer, FramebufferAbstract, Subpass};
use winit::{EventsLoop, WindowBuilder, Window};
use vulkano::sync::GpuFuture;
use vulkano::sync;
use vulkano::swapchain::{AcquireError};
use vulkano::swapchain;

use vulkano_win::{VkSurfaceBuild, CreationError as WindowCreationError};

use std::sync::Arc;

use crate::renderer::queues::{find_queues, Queues};
use crate::renderer::uniform_manager::UniformManager;
use crate::renderer::shader::ShaderSet;

/// Top level struct of vulkan renderer.
pub struct Renderer {
    settings: Rc<RefCell<Settings>>,
    instance: Arc<Instance>,
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    queues: Queues,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    uniform_manager: UniformManager,
    shader_set: Rc<ShaderSet>,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,

    recreate_swapchain: bool,
    previous_frame: Option<Box<GpuFuture>>,
}

impl Renderer {
    /// Creates new renderer.
    pub fn new(settings: Rc<RefCell<Settings>>, events_loop: &EventsLoop) -> Result<Self, RendererCreationError> {
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

        let surface = WindowBuilder::new().with_title(settings.borrow().window_title())
                                          .with_dimensions(settings.borrow().window_size().to_logical(settings.borrow().dpi()))
                                          .build_vk_surface(events_loop, instance.clone())?;
        let window = surface.window();

        window.grab_cursor(settings.borrow().grab_cursor()).unwrap();
        window.hide_cursor(settings.borrow().hide_cursor());

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

        let (swapchain, images) = {

            let capabilities = surface.capabilities(physical_device).expect("failed to get surface capabilities");
            let usage = capabilities.supported_usage_flags;
            let format = capabilities.supported_formats[0].0;

            let initial_dimensions = get_window_dimensions(settings.clone(), window);

            let present_mode = {
                if capabilities.present_modes.mailbox {
                    info!("Using Mailbox presentation mode");
                    PresentMode::Mailbox
                } else {
                    info!("Using Fifo presentation mode");
                    PresentMode::Fifo
                }
            };

            Swapchain::new(
                device.clone(),
                surface.clone(),
                capabilities.min_image_count,
                format,
                initial_dimensions,
                1,
                usage,
                &queues.graphics_queue(),
                SurfaceTransform::Identity,
                CompositeAlpha::Opaque,
                present_mode,
                true,
                None
            )
            .expect("failed to create swapchain")

        };

        let uniform_manager = UniformManager::new(device.clone());

        let shader_set = Rc::new(ShaderSet::load(device.clone()));

        let render_pass = Arc::new(
            single_pass_renderpass!(device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            ).expect("Couldn't create render pass")
        );

        let pipeline = create_pipeline(device.clone(), shader_set.clone(), &images, render_pass.clone());
        let framebuffers = create_framebuffers(device.clone(), &images, render_pass.clone());

        Ok(Renderer {
            settings,
            instance,
            surface,
            device: device.clone(),
            queues,
            swapchain,
            images,
            uniform_manager,
            shader_set,
            render_pass,
            pipeline,
            framebuffers,
            recreate_swapchain: false,
            previous_frame: None,
        })
    }

    pub fn render(&mut self, asset_manager: &mut AssetManager) {
        if let Some(previous_frame) = &mut self.previous_frame {
            previous_frame.cleanup_finished();
        }

        if self.recreate_swapchain {
            self.recreate_swapchain();
        }

        let (image_num, acquire_future) = match swapchain::acquire_next_image(self.swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                self.recreate_swapchain = true;
                return;
            }
            Err(err) => panic!("{:?}", err)
        };


        let mut command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queues.graphics_queue().family()).unwrap()
            .begin_render_pass(
                self.framebuffers[image_num].clone(), false,
                vec![
                    [0.0, 0.0, 0.0, 1.0].into(),
                ]
            ).unwrap();

        if let Some(scene) = asset_manager.active_scene() {
            let mut uniform_data = scene.camera().as_uniform_data();

            for object in scene.objects() {
                uniform_data.model = object.model_matrix().into();
                self.uniform_manager.update(uniform_data);
                let uniform_buffer_subbuffer = self.uniform_manager.get_subbuffer_data();

                let descriptor_set = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                    .add_buffer(uniform_buffer_subbuffer).unwrap()
                    .build().unwrap()
                );

                if let Some(mesh) = object.mesh() {
                    command_buffer = command_buffer.draw_indexed(
                        self.pipeline.clone(), 
                        &DynamicState::none(), 
                        vec!(mesh.vertex_buffer()),
                        mesh.index_buffer(), 
                        descriptor_set.clone(),
                        (),
                    ).unwrap();
                }
            }
        }

        let command_buffer = command_buffer.end_render_pass().unwrap().build().unwrap();

        let future = self.previous_frame.take()
                                        .unwrap_or_else(|| Box::new(sync::now(self.device.clone())) as Box<_>)
                                        .join(acquire_future)
                                        .then_execute(self.queues.graphics_queue(), command_buffer).unwrap()
                                        .then_swapchain_present(self.queues.graphics_queue(), self.swapchain.clone(), image_num)
                                        .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.previous_frame = Some(Box::new(future) as Box<_>);
            }
            Err(sync::FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }   
    }

    fn recreate_swapchain(&mut self) {
        let window_dimensions = get_window_dimensions(self.settings.clone(), self.surface.window());

        let (new_swapchain, new_images) = match self.swapchain.recreate_with_dimension(window_dimensions) {
            Ok(r) => r,
            Err(err) => panic!("{:?}", err)
        };

        self.swapchain = new_swapchain;
        self.images = new_images;

        self.pipeline = create_pipeline(self.device.clone(), self.shader_set.clone(), &self.images, self.render_pass.clone());
        self.framebuffers = create_framebuffers(self.device.clone(), &self.images, self.render_pass.clone());

        self.recreate_swapchain = false;
    }

    pub fn get_queues(&self) -> Queues {
        self.queues.clone()
    }

}

fn create_framebuffers(
    device: Arc<Device>, 
    images: &[Arc<SwapchainImage<Window>>], 
    render_pass: Arc<RenderPassAbstract + Send + Sync>
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    
    let dimensions = images[0].dimensions();

    let framebuffers = images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                        .add(image.clone()).unwrap()
                        .build().unwrap()
        ) as Arc<FramebufferAbstract + Send + Sync>}
    ).collect::<Vec<_>>();

    framebuffers
}

fn create_pipeline(
    device: Arc<Device>, 
    shader_set: Rc<ShaderSet>, 
    images: &[Arc<SwapchainImage<Window>>], 
    render_pass: Arc<RenderPassAbstract + Send + Sync>
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    
    let dimensions = images[0].dimensions();

    let pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_input(ShaderSet::vertex_layout())
        .vertex_shader(shader_set.vertex_shader().main_entry_point(), ())
        .triangle_list()
        .viewports_dynamic_scissors_irrelevant(1)
        .viewports(std::iter::once(Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0 .. 1.0,
        }))
        .fragment_shader(shader_set.fragment_shader().main_entry_point(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap());

    pipeline
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

fn get_window_dimensions(settings: Rc<RefCell<Settings>>, window: &Window) -> [u32; 2] {
    let dimensions = if let Some(dimensions) = window.get_inner_size() {
        let dimensions = dimensions.to_physical(settings.borrow().dpi());
        settings.borrow_mut().set_window_size(dimensions);
        let dimensions: (u32, u32) = dimensions.into();
        [dimensions.0, dimensions.1]
    } else {
        panic!("window was closed when calling get_window_dimensions");
    };

    dimensions
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