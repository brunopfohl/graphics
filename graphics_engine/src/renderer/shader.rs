use std::ffi::CString;
use gl::types::*;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Result<Shader, String> {
        let vertex_shader = Shader::compile_shader(vertex_src, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::compile_shader(fragment_src, gl::FRAGMENT_SHADER)?;
        
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // Check for linking errors
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), 
                                    buffer.as_mut_ptr() as *mut gl::types::GLchar);
                return Err(String::from_utf8_lossy(&buffer).to_string());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { id: program })
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, String> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source.as_bytes()).unwrap();
        
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(),
                                   buffer.as_mut_ptr() as *mut gl::types::GLchar);
                return Err(String::from_utf8_lossy(&buffer).to_string());
            }
        }

        Ok(shader)
    }
}
