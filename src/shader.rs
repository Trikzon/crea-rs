use std::io::Read;
use std::ffi::CString;
use std::{ptr, str};
use gl::types::*;

pub struct Shader {
    program_id: u32,
    gl: gl::Gl,
}

impl Shader {
    pub fn new(file_path: &str, gl: &gl::Gl) -> Self {
        // TODO: Move out to resource manager.
        let mut file = std::fs::File::open(file_path).expect("Unable to open file");
        let mut source = String::new();
        file.read_to_string(&mut source).expect("Unable to read the file");

        let mut shader_ids = Vec::new();
        for shader_type in ShaderType::split_source(&source) {
            shader_ids.push(shader_type.compile(gl));
        };

        let program_id = Self::compile(shader_ids, gl);

        Shader { program_id, gl: gl.clone()}
    }

    fn compile(shader_ids: Vec<u32>, gl: &gl::Gl) -> u32 {
        let program_id = unsafe { gl.CreateProgram() };
        for shader_id in &shader_ids {
            unsafe { gl.AttachShader(program_id, *shader_id); }
        }
        unsafe { gl.LinkProgram(program_id) }

        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl.GetProgramInfoLog(program_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }
        }
        for shader_id in shader_ids {
            unsafe {
                gl.DetachShader(program_id, shader_id);
                gl.DeleteShader(shader_id);
            }
        }
        program_id
    }

    pub fn enable(&self) {
        unsafe { self.gl.UseProgram(self.program_id) }
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

    fn compile(&self, gl: &gl::Gl) -> u32 {
        let (source, gl_type) = match self {
            Self::VERTEX(s) => (s, gl::VERTEX_SHADER),
            Self::FRAGMENT(s) => (s, gl::FRAGMENT_SHADER),
        };
        let id = unsafe { gl.CreateShader(gl_type) };
        let c_str_source = CString::new(source.as_bytes()).expect("Couldn't create CString.");

        unsafe {
            gl.ShaderSource(id, 1, &c_str_source.as_ptr(), ptr::null());
            gl.CompileShader(id);
        }

        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl.GetShaderInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::TYPE::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }
        }
        id
    }
}
