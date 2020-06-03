
use gl;
use gl::types::*;

use std::os::raw::c_void;
use std::ptr;
use std::mem;
use cgmath::prelude::Zero;
use cgmath::{ Vector3, Vector2 };
use std::ffi::{ CString, CStr };

use super::shader::Shader;

// REFACTOR PLEASE
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize as *const c_void
    }
}
//


#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
        }
    }
}

#[derive(Clone)]
pub struct Texture {
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
                        (self.vertices.len() * mem::size_of::<Vertex>()) as isize,
                        &self.indices[0] as *const u32 as *const c_void,
                        gl::STATIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        gl::BufferData( gl::ELEMENT_ARRAY_BUFFER,
                        (self.indices.len() * mem::size_of::<u32>()) as isize,
                        &self.indices[0] as *const u32 as *const c_void,
                        gl::STATIC_DRAW);


        let vertex_size = mem::size_of::<Vertex>() as i32;

        // vertex positions
        gl::EnableVertexAttribArray(0);	
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, position) as *const c_void);
        // vertex normals
        gl::EnableVertexAttribArray(1);	
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, normal) as *const c_void);
        // vertex texture coords
        gl::EnableVertexAttribArray(2);	
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, tex_coords) as *const c_void);

        gl::BindVertexArray(0);
    }

    pub unsafe fn draw(&self, shader: &Shader) {
        let mut diffuse_nr: u32 = 0;
        let mut specular_nr: u32 = 0;
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
            let material_CString = CString::new(format!("material.{}{}", name, number)).expect("CString::new failed");
            let material_CStr = CStr::from_bytes_with_nul_unchecked(material_CString.to_bytes_with_nul());
            shader.setFloat(material_CStr, i as f32);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }

        //draw mesh
        gl::BindVertexArray(self.VAO);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);

        gl::ActiveTexture(gl::TEXTURE0);
    }

}




