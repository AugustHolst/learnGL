#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::{CString, CStr};

mod shader;
use shader::Shader;

mod camera;
use camera::Camera;
use camera::Camera_Movement::*;

mod macros;

use cgmath::{Matrix4, Vector3, vec3, Point3, Deg, Rad, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled); // confines the mouse to only act on the program.

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let lightPos = vec3(1.0, 1.0, 0.0);
    let (lampShader, litShader, VBO, cubeVAO, lightVAO) = unsafe {
        
        gl::Enable(gl::DEPTH_TEST);
        
        let lampShader = Shader::new(
            "src/shaders/light.vert",
            "src/shaders/light.frag"
        );
        let litShader = Shader::new(
            "src/shaders/light.vert",
            "src/shaders/lit.frag"
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 108] = [
           -0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5,  0.5, -0.5,
            0.5,  0.5, -0.5,
           -0.5,  0.5, -0.5,
           -0.5, -0.5, -0.5,
       
           -0.5, -0.5,  0.5,
            0.5, -0.5,  0.5,
            0.5,  0.5,  0.5,
            0.5,  0.5,  0.5,
           -0.5,  0.5,  0.5,
           -0.5, -0.5,  0.5,
       
           -0.5,  0.5,  0.5,
           -0.5,  0.5, -0.5,
           -0.5, -0.5, -0.5,
           -0.5, -0.5, -0.5,
           -0.5, -0.5,  0.5,
           -0.5,  0.5,  0.5,
       
            0.5,  0.5,  0.5,
            0.5,  0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5,  0.5,
            0.5,  0.5,  0.5,
       
           -0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5,  0.5,
            0.5, -0.5,  0.5,
           -0.5, -0.5,  0.5,
           -0.5, -0.5, -0.5,
       
           -0.5,  0.5, -0.5,
            0.5,  0.5, -0.5,
            0.5,  0.5,  0.5,
            0.5,  0.5,  0.5,
           -0.5,  0.5,  0.5,
           -0.5,  0.5, -0.5,
        ];

        let (mut VBO, mut cubeVAO) = (0, 0);
        gl::GenVertexArrays(1, &mut cubeVAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(cubeVAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                    (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &vertices[0] as *const f32 as *const c_void,
                    gl::STATIC_DRAW);
        
        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        
        let mut lightVAO = 0;
        // light vao
        gl::GenVertexArrays(1, &mut lightVAO);
        gl::BindVertexArray(lightVAO);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        
        (lampShader, litShader, VBO, cubeVAO, lightVAO)
    };

    while !window.should_close() {
        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            litShader.useProgram();
            litShader.setVec3(c_str!("objectColor"), 1.0, 0.5, 0.31);
            litShader.setVec3(c_str!("lightColor"), 1.0, 1.0, 1.0);

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            litShader.setMat4(c_str!("projection"), &projection);
            litShader.setMat4(c_str!("view"), &view);

            // world transformation
            let mut model = Matrix4::<f32>::identity();
            model = model * Matrix4::from_scale(0.2);  // a smaller cube
            model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(20.0 * glfw.get_time() as f32));
            litShader.setMat4(c_str!("model"), &model);

            // render the cube
            gl::BindVertexArray(cubeVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // also draw the lamp object
            lampShader.useProgram();
            lampShader.setMat4(c_str!("projection"), &projection);
            lampShader.setMat4(c_str!("view"), &view);
            model = Matrix4::from_translation(lightPos);
            lampShader.setMat4(c_str!("model"), &model);

            gl::BindVertexArray(lightVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    // optional clean up.
    unsafe {
        gl::DeleteVertexArrays(1, &cubeVAO);
        gl::DeleteVertexArrays(1, &lightVAO);
        gl::DeleteBuffers(1, &VBO);
    }
}