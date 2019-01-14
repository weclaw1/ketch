use ketch_core::resource::mesh::Vertex;

pub fn generate_vertices() -> Vec<Vertex> {
    let vertices: Vec<Vertex> = vec![
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] },
        Vertex { position: [0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] },

        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] },
        Vertex { position: [0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] },

        Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0], tex_coord: [1.0, 0.0] },

        Vertex { position: [0.5,  0.5,  0.5], normal: [1.0, 0.0, 0.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [0.5,  0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [0.5, -0.5,  0.5], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] },
        Vertex { position: [0.5,  0.5,  0.5], normal: [1.0, 0.0, 0.0], tex_coord: [1.0, 0.0] },

        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 0.0] },
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 1.0] },

        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], tex_coord: [1.0, 0.0] },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 1.0] },
    ];

    vertices
}


pub fn generate_indices() -> Vec<u32> {
    let indices: Vec<u32> = vec![
        0, 1, 2,
        3, 4, 5,

        6, 7, 8,
        9, 10, 11,

        12, 13, 14,
        15, 16, 17,

        18, 19, 20,
        21, 22, 23,

        24, 25, 26,
        27, 28, 29,

        30, 31, 32,
        33, 34, 35,
    ];

    indices
}