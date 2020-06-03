use glfw::{ Context, Key, Action };

use std::sync::mpsc::Receiver;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub was_close_requested: bool,
    glfw: glfw::Glfw,
    glfw_window: glfw::Window,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
        let (mut glfw_window, event_receiver) = 
            glfw.create_window(width, height, "Rust GLFW", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
    
        glfw_window.make_current();
        glfw_window.set_key_polling(true);
        glfw_window.set_framebuffer_size_polling(true);
        gl::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);
    
        Window {
            width,
            height,
            was_close_requested: false,
            glfw,
            glfw_window,
            event_receiver
        }
    }

    pub fn process_events (&mut self) {
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                // Should move this into a function process_input. 
                // Or maybe split this function into checking both type of events.
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.was_close_requested = true,
                _ => {}
            }
        }
    }

    pub fn close(&mut self) {
        self.glfw_window.set_should_close(true)
    }

    pub fn get_time(&mut self) -> f32 {
        self.glfw.get_time() as f32
    }

    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }

    pub fn update(&mut self) -> () {
        self.glfw_window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
    }
}