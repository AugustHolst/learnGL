use engine::Engine;
use engine::EngineConfig;

fn main() {
    let config = EngineConfig {
        window_width: 800,
        window_height: 600
    };
    let engine = Engine::new(config);
    println!{"woo!"};
}
