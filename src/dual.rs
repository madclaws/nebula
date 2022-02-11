/* 
    Rendering 2 triangles using different shaders
*/

extern crate gl;
use crate::utils::*;
use std::os::raw::c_void;
use self::gl::types::*;
use std::mem;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CString;

pub struct Dual{
    vertices_a: [f32;18],
    vertices_b: [f32;9],
    shader_program: u32,
    shader_program_2: u32,
    VBO: u32,
    VBO_B: u32,
    VAO: u32,
    VAO_B: u32,
}

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

const FRAGMENT_SHADER_SOURCE_YELLOW: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
    }
"#;

const VERTEX_SHADER_SOURCE_MULTI_COLOR: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    out vec3 vertexColor;
    void main() {
        gl_Position = vec4(aPos, 1.0);
        vertexColor = aColor;
    }
"#;

const FRAGMENT_SHADER_SOURCE_B: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 vertexColor;
    void main() {
        FragColor = vec4(vertexColor, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE_UNIFORM: &str = r#"
    #version 330 core
    out vec4 FragColor;
    uniform vec4 ourColor;
    void main() {
        FragColor = ourColor;
    }
"#;

impl App for Dual {
    fn create() -> Self{
        let vertices_a: [f32; 18] = [-1.0, -0.5, 0.0, 1.0, 0.0, 0.0,
        0.0, -0.5, 0.0, 0.0, 1.0, 0.0,
        -0.5, 0.5, 0.0, 0.0, 0.0, 1.0];

        let vertices_b: [f32; 9] = [0.0, -0.5, 0.0, 1.0, -0.5, 0.0, 0.5, 0.5, 0.0];
    
        let mut VBO = 0;
        let mut VAO = 0;
        // let mut EBO = 0;
    
        let mut VBO_B = 0;
        let mut VAO_B = 0;
        
        let shader_program = create_shader_program(VERTEX_SHADER_SOURCE_MULTI_COLOR, FRAGMENT_SHADER_SOURCE_B);
        let shader_program_2 =
            create_shader_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE_YELLOW);
    
        render_triangle_a(&mut VBO, &mut VAO, &vertices_a);
        render_triangle(&mut VBO_B, &mut VAO_B, &vertices_b);
        Dual{vertices_a, vertices_b, shader_program, shader_program_2, VBO, VBO_B, VAO, VAO_B}
    }

    fn render(&mut self, time_value: f64) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.shader_program);

            let green_value = time_value.sin() / 2.0 + 0.5;
            let our_color = CString::new("ourColor").unwrap();
            let vertex_color_location = gl::GetUniformLocation(self.shader_program, our_color.as_ptr());
            println!("{green_value}");
            gl::Uniform4f(vertex_color_location, 0.0, green_value as f32, 0.0, 1.0);
            
            gl::BindVertexArray(self.VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(self.VAO_B);
            gl::UseProgram(self.shader_program_2);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
fn render_triangle_a(VBO: &mut u32, VAO: &mut u32, vertices: &[f32]) {
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
            6 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void
        );
        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    // *VAO
}