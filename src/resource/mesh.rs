use vulkano::buffer::BufferAccess;
use std::sync::Arc;

use vulkano::impl_vertex;
use vulkano::buffer::ImmutableBuffer;
use vulkano::buffer::BufferUsage;
use vulkano::device::Queue;

///Defines the information a Vertex should have
#[derive(Clone,Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl_vertex!(Vertex, position, color);

#[derive(Clone)]
pub struct Mesh {
    name: String,

    vertices: Vec<Vertex>,
    vertex_buffer: Arc<ImmutableBuffer<[Vertex]>>,

    indices: Vec<u32>,
    index_buffer: Arc<ImmutableBuffer<[u32]>>,
}

impl Mesh {
    pub fn new<S: Into<String>>(name: S, vertices: Vec<Vertex>, indices: Vec<u32>, upload_queue: Arc<Queue>) -> Self {
        let (vertex_buffer, _buffer_future) = ImmutableBuffer::from_iter(
            vertices.iter().cloned(),
            BufferUsage::all(),
            upload_queue.clone()
        ).expect("failed to create vertex buffer");

        let (index_buffer, _future) = ImmutableBuffer::from_iter(
            indices.iter().cloned(),
            BufferUsage::all(),
            upload_queue.clone()
        ).expect("failed to create index buffer");

        Mesh {
            name: name.into(),
            
            vertices: vertices,
            vertex_buffer: vertex_buffer,

            indices: indices,
            index_buffer: index_buffer,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn vertex_buffer(&self) -> Arc<ImmutableBuffer<[Vertex]>> {
        self.vertex_buffer.clone()
    }

    pub fn index_buffer(&self) -> Arc<ImmutableBuffer<[u32]>> {
        self.index_buffer.clone()
    }
}