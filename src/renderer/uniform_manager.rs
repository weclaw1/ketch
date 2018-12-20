use vulkano::memory::DeviceMemoryAllocError;
use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;
use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::buffer::BufferUsage;
use vulkano::device::Device;
use vulkano::memory::pool::StdMemoryPool;

use nalgebra_glm::Mat4;

use std::sync::Arc;

use crate::renderer::shader::vertex_shader::ty::TransformationData;

/// Struct which stores global uniform data and uniform buffer.
pub struct UniformManager {
    // data used in transformations (model, view, projection matrix)
    transformation_data: TransformationData,
    transformation_data_buffer_pool: CpuBufferPool<TransformationData>,
}

impl UniformManager {
    /// Creates new uniform manager.
    pub fn new(device: Arc<Device>) -> Self {
        let transformation_data = TransformationData {
            model: Mat4::identity().into(),
            view: Mat4::identity().into(),
            proj: Mat4::identity().into(),
        };

        let transformation_data_buffer_pool = CpuBufferPool::<TransformationData>::new(
            device.clone(), BufferUsage::all()
        );

        UniformManager {
            transformation_data: transformation_data,
            transformation_data_buffer_pool: transformation_data_buffer_pool,
        }
    }

    /// Updates uniform data.
    pub fn update(&mut self, transformation_data: TransformationData) {
        self.transformation_data = transformation_data;
    }

    /// Returns subbuffer from uniform buffer.
    pub fn get_subbuffer_data(&self) -> Result<CpuBufferPoolSubbuffer<TransformationData, Arc<StdMemoryPool>>, DeviceMemoryAllocError> {
        self.transformation_data_buffer_pool.next(self.transformation_data.clone())
    }
}

