use crate::window::Window;

enum GameState {
    Playing, Paused
}

pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32
}

pub struct Engine {
    window: Window,
    game_state: Option<GameState>
}

impl Engine {
    pub fn new(config: EngineConfig) -> Engine {
        let window = Window::new(config.window_width, config.window_height);
        Engine { 
            window,
            game_state: None
        }
    }
}