use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::buffer::BufferUsage;
use vulkano::device::Device;

use nalgebra_glm as glm;
use nalgebra_glm::Mat4;

use std::sync::Arc;

use crate::settings::Settings;
use crate::renderer::shader::vertex_shader::ty::TransformationData;

pub struct UniformManager {
    // data used in transformations (model, view, projection matrix)
    transformation_data: TransformationData,
    transformation_data_buffer_pool: CpuBufferPool<TransformationData>,
}

impl UniformManager {
    pub fn new(device: Arc<Device>) -> Self {
        let transformation_data = TransformationData {
            camera_position: [0.0; 3],
            _dummy0: [0; 4],
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
}

