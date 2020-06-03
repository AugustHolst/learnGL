use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use tobj;

use crate::model::mesh::{Mesh, Vertex, Texture};
use crate::model::Shader;

pub struct Scene {
    pub shader: Shader,
    pub root: Model
}

impl Scene {
    pub fn new(model_path: &str) -> Scene {

        // build and compile shaders
        // -------------------------
        let shader = Shader::new(
            "C:/Users/AdaMoe/Documents/rust_code/learnGL/engine/src/shaders/model.vert",
            "C:/Users/AdaMoe/Documents/rust_code/learnGL/engine/src/shaders/model.frag");

        // load models
        // -----------
        let root = Model::new(model_path);

        Scene {
            shader,
            root
        }
    }
}

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,
    directory: String
}

impl Model {
    pub fn new(path: &str) -> Model {
        let mut model = Model::default();
        model.load_model(path);
        model
    }

    pub fn draw(&self, shader: &Shader) -> () {
        for mesh in &self.meshes {
            unsafe { mesh.draw(shader); }
        }
    }

    fn load_model(&mut self, path: &str) -> () {
        let path = Path::new(path);
        // retrieve the directory path of the filepath
        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        println!("{}", &path.display());
        let obj = tobj::load_obj(&path, false);
        assert!(obj.is_ok());
        let (models, materials) = obj.unwrap();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            println!("Loading model with n: {} vertices", num_vertices);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position:  vec3(p[i*3], p[i*3+1], p[i*3+2]),
                    normal:    vec3(n[i*3], n[i*3+1], n[i*3+2]),
                    tex_coords: vec2(t[i*2], t[i*2+1])
                })
            }
            println!("Vertices pushed");
            // process material
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];
                
                // 1. diffuse map
                if !material.diffuse_texture.is_empty() {
                    println!("texture_diffuse");
                    let texture = self.loadMaterialTexture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    println!("texture_specular");
                    let texture = self.loadMaterialTexture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    println!("texture_normal");
                    let texture = self.loadMaterialTexture(&material.normal_texture, "texture_normal");
                    textures.push(texture);
                }
                // NOTE: no height maps
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    fn loadMaterialTexture(&mut self, path: &str, tex_type: &str) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|tex| tex.path == path);
            if let Some(texture) = texture {
                texture.clone();
            }
        }
        let texture = Texture {
            id: unsafe { TextureFromFile(path, &self.directory) },
            type_: tex_type.into(),
            path: path.into()
        };
        self.textures_loaded.push(texture.clone());
        texture
    }

}

unsafe fn TextureFromFile(path: &str, directory: &str) -> u32 {
    let filename = format!("{}/{}", directory, path);
    let mut textureID = 0;
    gl::GenTextures(1, &mut textureID);
    
    let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
    let img = img.flipv();
    
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
    };
    
    let data = img.raw_pixels();

    gl::BindTexture(gl::TEXTURE_2D, textureID);
    gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
        0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    textureID
}