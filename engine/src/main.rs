use engine::Engine;
use engine::EngineConfig;
use engine::PH;
use engine::model::mesh::Mesh;
use engine::model::mesh::Vertex;
use engine::model::mesh::Texture;
use cgmath::{Vector3, vec3, vec2};


// Main is purely for development, this should be a library.

fn main() {
    let config = EngineConfig {
        window_width: 800,
        window_height: 600
    };
    let mut engine = Engine::new(config);
    
    
    let vertex = Vertex { 
        position: vec3(0.0, 0.0, 0.0),
        normal: vec3(0.0, 0.0, 0.0),
        tex_coords: vec2(0.0, 1.0)
    };
    let texture = Texture {
        id: 0,
        type_: String::from("texture_diffuse"),
        path: String::from("C:\\Users\\AdaMoe\\Documents\\rust_code\\learnGL\\src\\resources\\deathpact-angel.jpg")
    };

    let (verts, indics, texs) = {
        let verts = vec![vertex];
        let indics = vec![1];
        let texs = vec![texture];
        (verts, indics, texs)
    };
    let mesh = Mesh::new(verts, indics, texs);
    
    let mut scene = PH { x: 0 };
    engine.start(&mut scene);

    println!{"woo!"};
}
