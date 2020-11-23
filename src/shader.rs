use gl::types::*;
use std::{ptr, str};
use std::ffi::CString;
use std::io::Read;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("compilation error in shader.\n{0}")]
    Compile(String),
    #[error("link error in shader.\n{0}")]
    Link(String),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
}

pub struct Shader {
    program_id: u32,
    gl: gl::Gl,
}

impl Shader {
    /// Takes a filepath to a glsl shader file then compiles and links the shader.
    /// The shader file should contain a fragment shader labeled with `#type fragment` and a vertex
    /// shader labeled with `#type vertex`.
    pub fn new(gl: &gl::Gl, file_path: &str) -> Result<Self, ShaderError> {
        // TODO: Move out to resource manager.
        let mut file = std::fs::File::open(file_path).expect("Unable to open file");
        let mut source = String::new();
        file.read_to_string(&mut source).expect("Unable to read the file");

        let mut shader_ids = Vec::new();
        for shader_type in ShaderType::split_source(&source) {
            shader_ids.push(shader_type.compile(gl)?);
        };
        let program_id = Self::compile(gl, shader_ids)?;

        Ok(Shader { program_id, gl: gl.clone() })
    }

    fn compile(gl: &gl::Gl, shader_ids: Vec<u32>) -> Result<u32, ShaderError> {
        let program_id = unsafe { gl.CreateProgram() };
        for shader_id in &shader_ids {
            unsafe { gl.AttachShader(program_id, *shader_id); }
        }
        unsafe { gl.LinkProgram(program_id) }
        check_shader_status(gl, program_id, 512, true)?;

        Ok(program_id)
    }

    pub fn enable(&self) {
        unsafe { self.gl.UseProgram(self.program_id); }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.program_id); }
    }
}

#[derive(Debug)]
enum ShaderType {
    FRAGMENT(String),
    VERTEX(String),
}

impl ShaderType {
    fn split_source(source: &str) -> Vec<ShaderType> {
        const TYPE_TOKEN: &str = "#type";
        const TYPE_TOKEN_LEN: usize = TYPE_TOKEN.len();
        let mut result = Vec::new();

        let mut starting_index = 0;
        while let Some(mut type_token_index) = source[starting_index..].find(TYPE_TOKEN) {
            type_token_index += TYPE_TOKEN_LEN + starting_index;

            let end_of_line_index = match source[type_token_index..].find("\n") {
                Some(i) => i + type_token_index,
                None => source.len() - 1,
            };
            let shader_type_token = source[type_token_index..end_of_line_index].trim();

            let type_source = if let Some(end_index) = source[end_of_line_index..].find(TYPE_TOKEN) {
                &source[end_of_line_index + 1..end_of_line_index + end_index]
            } else {
                &source[end_of_line_index + 1..]
            };

            if let Some(shader_type) = Self::from_token(shader_type_token, type_source) {
                result.push(shader_type);
            }
            starting_index = end_of_line_index + 1;
        }
        result
    }

    fn from_token(token: &str, source: &str) -> Option<ShaderType> {
        match token {
            "fragment" => Some(ShaderType::FRAGMENT(source.to_owned())),
            "vertex" => Some(ShaderType::VERTEX(source.to_owned())),
            _ => None
        }
    }

    fn compile(&self, gl: &gl::Gl) -> Result<u32, ShaderError> {
        let (source, gl_type) = match self {
            Self::VERTEX(s) => (s, gl::VERTEX_SHADER),
            Self::FRAGMENT(s) => (s, gl::FRAGMENT_SHADER),
        };

        let id = unsafe { gl.CreateShader(gl_type) };
        // TODO: Move to a tool module
        let c_str_source = CString::new(source.as_bytes()).expect("Couldn't create CString.");

        unsafe {
            gl.ShaderSource(id, 1, &c_str_source.as_ptr(), ptr::null());
            gl.CompileShader(id);
        }
        check_shader_status(gl, id, 512, false)?;

        Ok(id)
    }
}

fn check_shader_status(gl: &gl::Gl, id: u32, len: usize, shader_program: bool) -> Result<(), ShaderError> {
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(len + 1);
    info_log.extend([b' '].iter().cycle().take(len));

    if shader_program {
        unsafe { gl.GetProgramiv(id, gl::LINK_STATUS, &mut success); }
    } else {
        unsafe { gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }
    }

    if success != gl::TRUE as GLint {
        return if shader_program {
            unsafe {
                gl.GetProgramInfoLog(
                    id, len as i32, ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
            }
            Err(ShaderError::Link(str::from_utf8(&info_log)?.to_owned()))
        } else {
            unsafe {
                gl.GetShaderInfoLog(
                    id, len as i32, ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
            }
            Err(ShaderError::Compile(str::from_utf8(&info_log)?.to_owned()))
        }
    }
    Ok(())
}
