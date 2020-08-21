use crate::resources::Resources;
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub enum Error {
    ResourceError {
        name: String,
        inner: crate::resources::Error
    },
    CanNotDetermineShaderType {
        name: String,
    },
    CompileError {
        name: String,
        message: String,
    },
    LinkError {
        name: String,
        message: String,
    }
}

pub struct Shader {
    gl: gl::GL,
    id: gl::types::GLuint
}

impl Shader {
    pub fn load_shaders(gl: &gl::GL, res: &Resources, names: &[&str]) -> Result<Shader, Error> {
        const EXTENSIONS: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER)
        ];

        let mut shaders: Vec<gl::types::GLuint> = vec![];

        for name in names {
            let shader_type = EXTENSIONS
                .iter()
                .find(|&&(file_extension, _)| name.ends_with(file_extension))
                .map(|&(_, typ)| typ)
                .ok_or_else(|| Error::CanNotDetermineShaderType { name: name.to_owned().into() })?;

            let source = res.load_cstring(name).map_err(|e| Error::ResourceError {
                name: name.to_owned().into(),
                inner: e,
            })?;

            let id = unsafe { gl.CreateShader(shader_type) };
            unsafe {
                gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
                gl.CompileShader(id);
            }

            let mut success: gl::types::GLint = 1;
            unsafe {
                gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            }

            if success == 0 {
                let mut len: gl::types::GLint = 0;
                unsafe {
                    gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                }

                let error = create_whitespace_cstring_with_len(len as usize);

                unsafe {
                    gl.GetShaderInfoLog(
                        id, len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar
                    );
                }

                return Err(Error::CompileError {
                    name: name.to_owned().into(),
                    message: error.to_string_lossy().into_owned(),
                })
            }
            shaders.push(id);
        }
        let program_id = unsafe { gl.CreateProgram() };

        for shader in &shaders {
            unsafe {
                gl.AttachShader(program_id, shader.clone());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(Error::LinkError {
                name: "//TODO".to_owned(),
                message: error.to_string_lossy().into_owned(),
            });
        }

        for shader in &shaders {
            let shader = shader.clone();
            unsafe {
                gl.DetachShader(program_id, shader);
                gl.DeleteShader(shader);
            }
        }

        Ok(Shader {
            gl: gl.clone(),
            id: program_id,
        })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
