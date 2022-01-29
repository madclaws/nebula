extern crate glfw;
extern crate gl;
use self::glfw::{Action, Context, Key};
use std::mem;
use self::gl::types::*;
use std::os::raw::c_void;
use std::ffi::CString;
use std::ptr;

use std::sync::mpsc::Receiver;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

#[allow(non_snake_case)]
fn main() {
    // glfw initialize and configuration
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Creating the window
    let (mut window, events) = glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "NEBULA", glfw::WindowMode::Windowed)
    .expect("Failed to create the window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);


    // Load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _); 

    // Setting up vertex data for triangle
    let vertices: [f32;9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_vertex_source = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_vertex_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        // TODO: Validating the shader compilation
        let mut VBO = 0;
        gl::GenBuffers(1, &mut VBO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER, 
            vertices.len() as isize * mem::size_of::<GLfloat>() as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW 
        )
    }
    // render loop
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        process_input(&mut window, &events);
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {gl::Viewport(0, 0, width, height)}
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }

    }
}