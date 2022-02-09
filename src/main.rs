extern crate gl;
extern crate glfw;
use self::gl::types::*;
use self::glfw::{Action, Context, Glfw, Key, Window, WindowEvent};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
mod utils;
use utils::App;
mod dual;
use std::sync::mpsc::Receiver;
// use dual;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;


#[allow(non_snake_case)]
fn main() {
    let (mut window, events, mut glfw) = initialize_window();
    let mut app = dual::Dual::create();
    
    // render loop
    while !window.should_close() {
        let time_value = glfw.get_time();
        app.render(time_value);
        process_input(&mut window, &events);
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}

#[allow(non_snake_case)]
fn render_triangle(VBO: &mut u32, VAO: &mut u32, vertices: &[f32]) {
    unsafe {
        gl::GenBuffers(1, VBO);
        // gl::GenBuffers(1, &mut EBO);
        gl::GenVertexArrays(1, VAO);

        gl::BindVertexArray(*VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, *VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            vertices.len() as isize * mem::size_of::<GLfloat>() as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    // *VAO
}

#[allow(non_snake_case)]
fn render_rectangle(
    VBO: &mut u32,
    EBO: &mut u32,
    VAO: &mut u32,
    vertices: &[f32],
    indices: &[i32],
) -> u32 {
    unsafe {
        gl::GenBuffers(1, VBO);
        gl::GenBuffers(1, EBO);
        gl::GenVertexArrays(1, VAO);

        gl::BindVertexArray(*VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, *VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            vertices.len() as isize * mem::size_of::<GLfloat>() as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *EBO);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            indices.len() as isize * mem::size_of::<GLfloat>() as GLsizeiptr,
            &indices[0] as *const i32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    *VAO
}

fn create_shader_program(vertex_shader_src: &str, fragment_shader_src: &str) -> u32 {
    // vertex shader
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_vertex_source = CString::new(vertex_shader_src.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_vertex_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as GLint;
        let mut info_log: Vec<u8> = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED {:?}",
                str::from_utf8(&info_log).unwrap()
            );
        }

        //  fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_fragment_source = CString::new(fragment_shader_src.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_fragment_source.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        let mut success_fragment = gl::FALSE as GLint;
        let mut info_log_fragment: Vec<u8> = Vec::with_capacity(512);
        info_log_fragment.set_len(512 - 1);
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success_fragment);
        if success_fragment != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                ptr::null_mut(),
                info_log_fragment.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED {:?}",
                str::from_utf8(&info_log_fragment).unwrap()
            );
        }

        // Shader program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success_fragment);
        if success_fragment != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                ptr::null_mut(),
                info_log_fragment.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED {:?}",
                str::from_utf8(&info_log_fragment).unwrap()
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}

/// Setup up the renderer window using GLFW
/// Returns window and events references
fn initialize_window() -> (Window, Receiver<(f64, WindowEvent)>, Glfw) {
    // glfw initialize and configuration
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Creating the window
    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "NEBULA",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create the window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    (window, events, glfw)
}
