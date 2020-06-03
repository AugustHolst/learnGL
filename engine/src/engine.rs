use crate::*;
use cgmath::{ Deg, perspective, Vector3, Matrix4, Point3};
use std::ffi::{CString, CStr};

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
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
    
    pub fn start(&mut self, scene: &mut Scene) -> () {
        self.run_loop(scene);
    }
    
    fn run_loop(&mut self, scene: &mut Scene) -> () {
        unsafe { gl::Enable(gl::DEPTH_TEST); } 

        let (eye, tar, up) = {
            let eye = Point3::<f32>::new(0.0, 0.0, 0.0);
            let tar = Point3::<f32>::new(0.0, 0.0, 0.0);
            let up = Vector3::<f32>::unit_y();
            (eye, tar, up)
        };

        let mut model_pos = Vector3::<f32>::new(0.0, 0.0, 0.0);

        let mut last_frame: f32 = 0.0;

        while !self.window.should_close() {
            let curr_frame = self.window.get_time() as f32;
            let delta_time = curr_frame - last_frame;
            let last_frame = curr_frame;

            self.window.process_events();

            //render
            unsafe {
                self.window.clear();
                
                scene.shader.useProgram();

                let projection: Matrix4::<f32> = perspective(Deg(45.0), self.window.width as f32 / self.window.height as f32, 0.1, 100.0);
                let view: Matrix4<f32> = Matrix4::from_translation(Vector3::<f32>::new(0., 0., -3.));
                scene.shader.setMat4(c_str!("projection"), &projection);
                scene.shader.setMat4(c_str!("view"), &view);

                let mut model = Matrix4::<f32>::from_translation(model_pos);
                scene.shader.setMat4(c_str!("model"), &model);
                scene.root.draw(&scene.shader);

                self.window.update();
            }
        }
    }
}
