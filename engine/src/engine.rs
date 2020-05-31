use crate::window::Window;
pub struct PH {
    pub x: i32
}

pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32
}

pub struct Engine {
    window: Window
}

impl Engine {
    pub fn new(config: EngineConfig) -> Engine {
        let window = Window::new(config.window_width, config.window_height);
        Engine { 
            window,
        }
    }
    
    pub fn start(&mut self, scene: &mut PH) -> () {
        //scene.init();

        self.run_loop(scene);

        self.window.close();
    }
    
    fn run_loop(&mut self, scene: &mut PH) {
        
    }
}