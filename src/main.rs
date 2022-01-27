extern crate glfw;
extern crate gl;
use self::glfw::{Action, Context, Key};

use std::sync::mpsc::Receiver;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

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