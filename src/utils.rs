/// Module for common utils

// We need to create a trait, that then will be implemented by all the 
// apps who want to render something in the main.
extern crate gl;
extern crate glfw;

use self::gl::types::*;
use std::ffi::CString;
use std::str;
use std::ptr;

pub trait App {
    fn create() -> Self;
    fn render(&mut self);
}

pub fn create_shader_program(vertex_shader_src: &str, fragment_shader_src: &str) -> u32 {
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