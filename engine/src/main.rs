use engine::Engine;
use engine::EngineConfig;
use engine::PH;


fn main() {
    let config = EngineConfig {
        window_width: 800,
        window_height: 600
    };
    let mut engine = Engine::new(config);
    
    let mut scene = PH { x: 0 };
    engine.start(&mut scene);
    
    let vertex = Vertex { 
        position: vec3(0.0, 0.0, 0.0),
        normal: vec3(0.0, 0.0, 0.0),
        texcoords
    };

    let (vertices, indics, texco) = {
        verts = Vec![Vector::]
    };
    //let mesh = Mesh::new()
    
    println!{"woo!"};
}
