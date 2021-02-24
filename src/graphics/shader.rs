use crate::graphics::gl;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("#type '{0}' is invalid")]
    InvalidTypeHeader(String),
    #[error("missing #type annotation in multi-shader")]
    MissingTypeHeader,
    #[error("encountered a gl error")]
    GLError(#[from] gl::GLError),
}

pub enum ShaderSource {
    VERTEX(String),
    FRAGMENT(String),
}

pub struct ShaderProgram {
    program_id: gl::ProgramId,
}

impl ShaderProgram {
    pub fn from_shaders(sources: &Vec<ShaderSource>) -> Result<Self, ShaderError> {
        let shaders = sources
            .iter()
            .map(|source| {
                let (shader_id, source) = match source {
                    ShaderSource::VERTEX(source) => {
                        (gl::create_shader(gl::ShaderType::Vertex)?, source)
                    }
                    ShaderSource::FRAGMENT(source) => {
                        (gl::create_shader(gl::ShaderType::Fragment)?, source)
                    }
                };

                gl::set_shader_source(&shader_id, source)?;
                gl::compile_shader(&shader_id)?;

                Ok(shader_id)
            })
            .collect::<Result<Vec<gl::ShaderId>, gl::GLError>>()?;

        let program_id = gl::create_program()?;

        for shader_id in &shaders {
            gl::attach_shader(&program_id, &shader_id);
        }

        gl::link_program(&program_id)?;

        for shader_id in &shaders {
            gl::detach_shader(&program_id, &shader_id);
            gl::delete_shader(&shader_id);
        }

        Ok(ShaderProgram { program_id })
    }

    pub fn from_multi_shader(multi_shader: &str) -> Result<Self, ShaderError> {
        let mut shaders: Vec<ShaderSource> = Vec::new();
        for shader in multi_shader.split("#type ") {
            if shader.trim().is_empty() {
                continue;
            }

            let end_of_type_header = match shader.find(|s| s == ' ' || s == '\n') {
                Some(index) => index,
                None => return Err(ShaderError::MissingTypeHeader),
            };
            let type_header = shader[0..end_of_type_header].trim();

            let source = &shader[end_of_type_header + 1..];

            match type_header {
                "vertex" => shaders.push(ShaderSource::VERTEX(source.to_owned())),
                "fragment" => shaders.push(ShaderSource::FRAGMENT(source.to_owned())),
                _ => return Err(ShaderError::InvalidTypeHeader(type_header.to_owned())),
            }
        }
        ShaderProgram::from_shaders(&shaders)
    }

    #[inline]
    pub fn bind(&self) {
        gl::bind_program(&self.program_id);
    }

    #[inline]
    pub fn unbind(&self) {
        gl::unbind_program();
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.unbind();
        gl::delete_program(&self.program_id);
    }
}
