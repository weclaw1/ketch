use vulkano::command_buffer::ExecuteCommandsError;
use vulkano::image::ImageCreationError;
use vulkano::sync::FlushError;
use vulkano::command_buffer::CommandBufferExecError;
use vulkano::command_buffer::BuildError;
use vulkano::command_buffer::AutoCommandBufferBuilderContextError;
use vulkano::command_buffer::DrawIndexedError;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuildError;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetError;
use vulkano::memory::DeviceMemoryAllocError;
use derive_error::Error;

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

#[derive(Debug, Error)]
pub enum RendererCreationError {
    InstanceCreationError(InstanceCreationError),
    WindowCreationError(WindowCreationError),
    DeviceCreationError(DeviceCreationError),
    CapabilitiesError(CapabilitiesError),
    SwapchainCreationError(SwapchainCreationError),
    GraphicsPipelineCreationError(GraphicsPipelineCreationError),
    FramebufferCreationError(FramebufferCreationError),
    RenderPassCreationError(RenderPassCreationError),

    /// Couldn't find usable physical device.
    #[error(no_from, non_std)]
    NoPhysicalDeviceError,
} 


#[derive(Debug, Error)]
pub enum RenderError {
    SwapchainCreationError(SwapchainCreationError),
    GraphicsPipelineCreationError(GraphicsPipelineCreationError),
    FramebufferCreationError(FramebufferCreationError),
    AcquireError(AcquireError),
    OomError(OomError),
    BeginRenderPassError(BeginRenderPassError),
    DeviceMemoryAllocError(DeviceMemoryAllocError),
    PersistentDescriptorSetError(PersistentDescriptorSetError),
    PersistentDescriptorSetBuildError(PersistentDescriptorSetBuildError),
    DrawIndexedError(DrawIndexedError),
    AutoCommandBufferBuilderContextError(AutoCommandBufferBuilderContextError),
    CommandBufferBuildError(BuildError),
    CommandBufferExecError(CommandBufferExecError),
    FlushError(FlushError),
    ExecuteCommandsError(ExecuteCommandsError),
}