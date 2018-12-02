use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;
use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::buffer::BufferUsage;
use vulkano::device::Device;
use vulkano::memory::pool::StdMemoryPool;

use nalgebra_glm as glm;
use nalgebra_glm::Mat4;

use std::sync::Arc;

use crate::renderer::shader::vertex_shader::ty::TransformationData;

use log::*;

pub struct UniformManager {
    // data used in transformations (model, view, projection matrix)
    transformation_data: TransformationData,
    transformation_data_buffer_pool: CpuBufferPool<TransformationData>,
}

impl UniformManager {
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

    pub fn update(&mut self, transformation_data: TransformationData) {
        self.transformation_data = transformation_data;
    }

    pub fn get_subbuffer_data(&self) -> CpuBufferPoolSubbuffer<TransformationData, Arc<StdMemoryPool>> {
        match self.transformation_data_buffer_pool.next(self.transformation_data.clone()){
            Ok(buffer) => buffer,
            Err(error) => {
                error!("{:?}", error);
                panic!("failed to allocate new subbuffer!")
            },
        }
    }
}

