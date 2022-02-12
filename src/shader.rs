/* 
    Shader module
*/
extern crate gl;
use std::fs;
use std::ffi::CString;
use std::ptr;
use self::gl::types::*;
use std::str;

pub struct Shader {
    id: u32
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        Shader{id: create_program(vertex_path, fragment_path)}
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id)
        }       
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        unsafe {
            let name_c_str = CString::new(name).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.id, name_c_str.as_ptr()), value as GLint)
        }
    }

    pub fn set_int(&mut self, name: &str, value: u32) {
        unsafe {
            let name_c_str = CString::new(name).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.id, name_c_str.as_ptr()), value as GLint)
        }
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        unsafe {
            let name_c_str = CString::new(name).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.id, name_c_str.as_ptr()), value as f32)
        }
    }
}

fn create_program(vertex_path: &str, fragment_path: &str) -> u32{
    let vertex_shader_pgm = fs::read_to_string(vertex_path)
    .expect("Unable to load vertex shader program");

    let fragment_shader_pgm = fs::read_to_string(fragment_path)
    .expect("Unable to load vertex shader program");

    let shader_program: u32;
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_vertex_source = CString::new(vertex_shader_pgm.as_bytes()).unwrap();
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
         let c_fragment_source = CString::new(fragment_shader_pgm.as_bytes()).unwrap();
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
        shader_program = gl::CreateProgram();
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
    }
    shader_program
}

