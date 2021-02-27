use crate::graphics::gl;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("#type '{0}' is invalid")]
    InvalidTypeHeader(String),
    #[error("missing #type annotation in multi-shader")]
    MissingTypeHeader,
    #[error("attempted to upload to an undefined uniform")]
    UndefinedUniform,
    #[error("encountered a gl error")]
    GLError(#[from] gl::GLError),
}

pub enum ShaderSource {
    VERTEX(String),
    FRAGMENT(String),
}

pub struct ShaderProgram {
    program_id: gl::ProgramId,
    uniform_locations: HashMap<String, gl::UniformLocation>,
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

        Ok(ShaderProgram {
            program_id,
            uniform_locations: HashMap::new(),
        })
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

    pub fn define_uniform(&mut self, name: &str) -> Result<(), ShaderError> {
        match gl::get_uniform_location(&self.program_id, name) {
            Ok(location) => {
                self.uniform_locations.insert(name.to_owned(), location);
                Ok(())
            }
            Err(e) => Err(ShaderError::GLError(e)),
        }
    }

    pub fn upload_uniform(
        &self,
        name: &str,
        data: &impl gl::UniformData,
    ) -> Result<(), ShaderError> {
        if let Some(location) = self.uniform_locations.get(name) {
            data.upload(location);
            Ok(())
        } else {
            Err(ShaderError::UndefinedUniform)
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.unbind();
        gl::delete_program(&self.program_id);
    }
}
