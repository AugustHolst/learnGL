use engine::Engine;
use engine::EngineConfig;
use engine::model::*;
use std::path::Path;
use cgmath::{Vector3, vec3, vec2};
use tobj;

// Main is purely for development, this should be a library.

fn main() {
    let config = EngineConfig {
        window_width: 800,
        window_height: 600
    };
    let mut engine = Engine::new(config);
    let mut scene = Scene::new("src/ico_sphere/b_cube.obj");
    engine.start(&mut scene);
}
