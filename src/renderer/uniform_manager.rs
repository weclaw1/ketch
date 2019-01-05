use vulkano::memory::DeviceMemoryAllocError;
use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;
use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::buffer::BufferUsage;
use vulkano::device::Device;
use vulkano::memory::pool::StdMemoryPool;

use nalgebra_glm::{Vec3, Mat4};

use std::sync::Arc;

use crate::renderer::shader::vertex_shader::ty::TransformationData;
use crate::renderer::shader::fragment_shader::ty::LightData;

/// Struct which stores uniform data and uniform buffers.
#[derive(Clone)]
pub struct UniformManager {
    // data used in transformations (model, view, projection matrix)
    transformation_data: TransformationData,
    transformation_data_buffer_pool: CpuBufferPool<TransformationData>,

    // data used with lighting
    light_data: LightData,
    light_data_buffer_pool: CpuBufferPool<LightData>,
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

        let light_data = LightData {
            _dummy0: [0; 4],
            light_position: Vec3::new(0.0, 0.0, 0.0).into(),
            light_color: Vec3::new(1.0, 1.0, 1.0).into(),
        };

        let light_data_buffer_pool = CpuBufferPool::<LightData>::new(device.clone(), BufferUsage::all());

        UniformManager {
            transformation_data,
            transformation_data_buffer_pool,
            light_data,
            light_data_buffer_pool,
        }
    }

    /// Updates light uniform data.
    pub fn update_light_data(&mut self, light_data: LightData) {
        self.light_data = light_data;
    }

    /// Returns subbuffer from light uniform buffer.
    pub fn get_light_subbuffer_data(&self) -> Result<CpuBufferPoolSubbuffer<LightData, Arc<StdMemoryPool>>, DeviceMemoryAllocError> {
        self.light_data_buffer_pool.next(self.light_data.clone())
    }

    /// Updates transformation uniform data.
    pub fn update_transformation_data(&mut self, transformation_data: TransformationData) {
        self.transformation_data = transformation_data;
    }

    /// Returns subbuffer from transformation uniform buffer.
    pub fn get_transformation_subbuffer_data(&self) -> Result<CpuBufferPoolSubbuffer<TransformationData, Arc<StdMemoryPool>>, DeviceMemoryAllocError> {
        self.transformation_data_buffer_pool.next(self.transformation_data.clone())
    }
}

