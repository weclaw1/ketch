pub mod vertex_shader;
pub mod fragment_shader;

use std::rc::Rc;
use vulkano::device::Device;
use std::sync::Arc;

use vulkano::pipeline::vertex::SingleBufferDefinition;

use crate::resource::mesh::Vertex;

pub struct ShaderSet {
    vertex_shader: vertex_shader::Shader,
    fragment_shader: fragment_shader::Shader,
}

impl ShaderSet {
    pub fn load(device: Arc<Device>) -> Self {

        let v_s = vertex_shader::Shader::load(device.clone()).expect("failed to load vertex shader!");
        let f_s = fragment_shader::Shader::load(device.clone()).expect("failed to load vertex shader!");

        ShaderSet {
            vertex_shader: v_s,
            fragment_shader: f_s,
        }
    }

    pub fn vertex_layout() -> SingleBufferDefinition<Vertex> {
        SingleBufferDefinition::<Vertex>::new()
    }

    pub fn vertex_shader(&self) -> &vertex_shader::Shader {
        &self.vertex_shader
    }

    pub fn fragment_shader(&self) -> &fragment_shader::Shader {
        &self.fragment_shader
    }
}