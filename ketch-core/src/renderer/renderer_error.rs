use std::error::Error;

use vulkano::sync::FlushError;
use vulkano::command_buffer::CommandBufferExecError;
use vulkano::command_buffer::BuildError;
use vulkano::command_buffer::AutoCommandBufferBuilderContextError;
use vulkano::command_buffer::DrawIndexedError;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuildError;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetError;
use vulkano::memory::DeviceMemoryAllocError;

use vulkano::command_buffer::BeginRenderPassError;
use vulkano::OomError;
use vulkano::swapchain::AcquireError;
use vulkano::framebuffer::FramebufferCreationError;
use vulkano::pipeline::GraphicsPipelineCreationError;
use vulkano::framebuffer::RenderPassCreationError;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::swapchain::CapabilitiesError;
use vulkano::device::DeviceCreationError;
use vulkano::instance::InstanceCreationError;
use vulkano_win::{CreationError as WindowCreationError};

use quick_error::quick_error; 

quick_error! {
    #[derive(Debug)]
    pub enum RenderError {
        SwapchainCreationError(err: SwapchainCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        GraphicsPipelineCreationError(err: GraphicsPipelineCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        FramebufferCreationError(err: FramebufferCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        AcquireError(err: AcquireError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        OomError(err: OomError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        BeginRenderPassError(err: BeginRenderPassError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        DeviceMemoryAllocError(err: DeviceMemoryAllocError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        PersistentDescriptorSetError(err: PersistentDescriptorSetError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        PersistentDescriptorSetBuildError(err: PersistentDescriptorSetBuildError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        DrawIndexedError(err: DrawIndexedError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        AutoCommandBufferBuilderContextError(err: AutoCommandBufferBuilderContextError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        CommandBufferBuildError(err: BuildError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        CommandBufferExecError(err: CommandBufferExecError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        FlushError(err: FlushError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum RendererCreationError {
        InstanceCreationError(err: InstanceCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        WindowCreationError(err: WindowCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        DeviceCreationError(err: DeviceCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        CapabilitiesError(err: CapabilitiesError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        SwapchainCreationError(err: SwapchainCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        GraphicsPipelineCreationError(err: GraphicsPipelineCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        FramebufferCreationError(err: FramebufferCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        RenderPassCreationError(err: RenderPassCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        NoPhysicalDeviceError {
            display("NoPhysicalDeviceError: couldn't find usable physical device")
        }
    } 
}