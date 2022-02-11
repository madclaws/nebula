/* 
    Shader module
*/
extern crate gl;
use std::fs;
use std::ffi::CString;
use std::ptr;
use self::gl::types::*;

pub struct Shader {
    id: u32
}

impl Shader {
    fn new(vertex_path: &str, fragment_path: &str) -> Self {
        Shader{id: 1}
    }
}

fn create_program(vertex_path: &str, fragment_path: &str) -> u32{
    let vertex_shader_pgm = fs::read_to_string(vertex_path)
    .expect("Unable to load vertex shader program");

    let fragment_shader_pgm = fs::read_to_string(fragment_path)
    .expect("Unable to load vertex shader program");

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_vertex_source = CString::new(vertex_path.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_vertex_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as GLint;
        let mut info_log: Vec<u8> = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        // if success != gl::TRUE as GLint {
        //     gl::GetShaderInfoLog(
        //         vertex_shader,
        //         512,
        //         ptr::null_mut(),
        //         info_log.as_mut_ptr() as *mut GLchar,
        //     );
        //     println!(
        //         "ERROR::SHADER::VERTEX::COMPILATION_FAILED {:?}",
        //         str::from_utf8(&info_log).unwrap()
        //     );
        // }
    }
    2

}

