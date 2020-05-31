
extern crate gl;
use self::gl::types::*;

extern crate glfw;
use self::glfw::{Context, Key, Action};

use std::os::raw::c_void;
use std::ptr;
use std::mem;
use cgmath::{ Vector3, vec3, Vector2, vec2 };
use std::ffi::{CString, CStr};

use super::shader::Shader;
// REFACTOR PLEASE

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize as *const c_void
    }
}
//



pub struct Vertex { //should be private
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>
}

pub struct Texture { //should be private
    pub id: u32,
    pub type_: String,
    pub path: String
}



pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub VAO: u32, 

    VBO: u32, 
    EBO: u32
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            VAO: 0, VBO: 0, EBO: 0
        };

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        
        unsafe { mesh.setup_mesh() }
        mesh
    }

    unsafe fn setup_mesh(&mut self) {
        gl::GenVertexArrays(1, &mut self.VAO);
        gl::GenBuffers(1, &mut self.VBO);
        gl::GenBuffers(1, &mut self.EBO);

        gl::BindVertexArray(self.VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);

        gl::BufferData( gl::ARRAY_BUFFER, 
                        (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &self.indices[0] as *const u32 as *const c_void,
                        gl::STATIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        gl::BufferData( gl::ELEMENT_ARRAY_BUFFER,
                        (self.indices.len() * mem::size_of::<GLfloat>()) as isize,
                        &self.indices[0] as *const u32 as *const c_void,
                        gl::STATIC_DRAW);


        let vertex_size = mem::size_of::<Vertex>() as i32;
        // vertex positions
        gl::EnableVertexAttribArray(0);	
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, position));
        // vertex normals
        gl::EnableVertexAttribArray(1);	
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, normal));
        // vertex texture coords
        gl::EnableVertexAttribArray(2);	
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, tex_coords));

        gl::BindVertexArray(0);
    }

    pub unsafe fn draw(&self, shader: &Shader) {
        let mut diffuse_nr: u32 = 1;
        let mut specular_nr: u32 = 1;
        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);

            let name = &texture.type_;
            let number = match name.as_str() {
                "texture_diffuse" => {
                    diffuse_nr += 1;
                    diffuse_nr
                }
                "texture_specular" => {
                    specular_nr += 1;
                    specular_nr
                }
                _ => panic!("unknown texture type")
            };

            let sampler = CString::new(format!("{}{}", name, number)).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(shader.ID, sampler.as_ptr()), i as i32);

            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }


        //draw mesh
        gl::BindVertexArray(self.VAO);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        
        gl::BindVertexArray(0);
        gl::ActiveTexture(gl::TEXTURE0);
    }

}




