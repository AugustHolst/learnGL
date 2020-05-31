use cgmath::{Vector3, vec3, vec2};
use gl::types::*;

pub struct Vertex {
    position: Vector3,
    normal: Vector3,
    tex_coords: vec2
}

pub struct Texture {
    id: u32,
    tex_type: String
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    VAO: u32, 
    VBO: u32, 
    EBO: u32
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let new_mesh = Mesh {
            vertices,
            indices,
            textures,
        }.setup_mesh();
    }

    fn setup_mesh(&mut self) {
        gl::GenVertexArrays(1, &self.VAO);
        gl::GenBuffers(1, &self.VBO);
        gl::GenBuffers(1, &self.EBO);

        gl::BindVertexArray(VAO);
        gl::BindBuffer(GL_ARRAY_BUFFER, VBO);

        gl::BufferData( GL_ARRAY_BUFFER, 
                        (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &self.vertices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);
        
        gl::BindBuffer(GL_ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData( GL_ELEMENT_ARRAY_BUFFER,
                        (self.indices.len() * mem::size_of::<GLfloat>()),
                        &self.indices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

                        //

        gl::BindVertexArray(0);
    }
}

