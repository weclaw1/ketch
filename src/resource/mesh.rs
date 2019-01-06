use crate::resource::texture::Texture;
use std::sync::Arc;

use vulkano::impl_vertex;
use vulkano::buffer::ImmutableBuffer;
use vulkano::buffer::BufferUsage;
use vulkano::device::Queue;

///Defines the information a Vertex should have
#[derive(Clone,Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl_vertex!(Vertex, position, normal, tex_coord);

/// Mesh is a collection of vertices, edges and faces that defines shape of object.
pub struct Mesh {
    name: String,

    vertices: Vec<Vertex>,
    vertex_buffer: Arc<ImmutableBuffer<[Vertex]>>,

    indices: Vec<u32>,
    index_buffer: Arc<ImmutableBuffer<[u32]>>,

    texture: Arc<Texture>,
}

impl Mesh {
    /// Creates new mesh.
    pub fn new<S: Into<String>>(name: S, vertices: Vec<Vertex>, indices: Vec<u32>, texture: Arc<Texture>, upload_queue: Arc<Queue>) -> Self {
        let (vertex_buffer, _buffer_future) = ImmutableBuffer::from_iter(
            vertices.iter().cloned(),
            BufferUsage::all(),
            upload_queue.clone()
        ).expect("failed to create vertex buffer");

        let (index_buffer, _future) = ImmutableBuffer::from_iter(
            indices.iter().cloned(),
            BufferUsage::all(),
            upload_queue
        ).expect("failed to create index buffer");

        Mesh {
            name: name.into(),
            
            vertices: vertices,
            vertex_buffer: vertex_buffer,

            indices: indices,
            index_buffer: index_buffer,

            texture,
        }
    }

    /// Returns the name of this mesh.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets texture used by this mesh.
    pub fn set_texture(&mut self, texture: Arc<Texture>) {
        self.texture = texture;
    }

    /// Returns texture used by this mesh.
    pub fn texture(&self) -> Arc<Texture> {
        self.texture.clone()
    }

    /// Returns the vertex buffer of this mesh.
    pub fn vertex_buffer(&self) -> Arc<ImmutableBuffer<[Vertex]>> {
        self.vertex_buffer.clone()
    }

    /// Returns the index buffer of this mesh.
    pub fn index_buffer(&self) -> Arc<ImmutableBuffer<[u32]>> {
        self.index_buffer.clone()
    }
}