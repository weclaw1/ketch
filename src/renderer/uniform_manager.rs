use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::device::Device;

use nalgebra_glm as glm;

use std::sync::Arc;

use crate::settings::Settings;

pub struct UniformManager {
    // data used in transformations (model, view, projection matrix)
    transformation_data: TransformationData,
    transformation_data_buffer_pool: CpuBufferPool<TransformationData>,
}

impl UniformManager {
    pub fn new(device: Arc<Device>, settings: &Settings) {

    }
}

