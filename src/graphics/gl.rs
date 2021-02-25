use crate::vector::*;
use bitflags::bitflags;
use raw_gl::types::*;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum GLError {
    #[error("OpenGL failed to create a new shader object")]
    CreateShader,
    #[error("OpenGL failed to compile the given shader's source. Info Log: {0}")]
    CompileShader(String),

    #[error("OpenGL failed to create a new program object")]
    CreateProgram,
    #[error("OpenGL failed to link the given program's shaders. Info Log: {0}")]
    LinkProgram(String),

    #[error("given uniform name is invalid")]
    InvalidUniformName(String),

    #[error("failed to convert &str into CString because it contains an interior nul byte")]
    NulByteInStr(#[from] std::ffi::NulError),
    #[error("failed to get Utf8 str from OpenGL")]
    Utf8(#[from] std::str::Utf8Error),
}

/// Load each OpenGL symbol using a custom load function.
///
/// ```ignore
/// crean::gl::load_with(|s| glfw_window.get_proc_address(s));
/// ```
pub fn load_with<F>(load_fn: F)
where
    F: FnMut(&'static str) -> *const GLvoid,
{
    raw_gl::load_with(load_fn);
}

bitflags! {
    pub struct ClearFlag: u32 {
         const COLOR_BUFFER = raw_gl::COLOR_BUFFER_BIT;
         const DEPTH_BUFFER = raw_gl::DEPTH_BUFFER_BIT;
         const STENCIL_BUFFER = raw_gl::STENCIL_BUFFER_BIT;
    }
}

/// Specifies the red, green, blue, and alpha values used by `gl::clear()` to
/// clear the color buffers. Values are clamped to the range [0, 1].
#[inline]
pub fn set_clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { raw_gl::ClearColor(red, green, blue, alpha) };
}

/// Sets the bitplane area of the window to values previously selected by
/// `gl::clear_color()`, `gl::clear_depth()`, and `gl::clear_stencil()`.
///
/// Takes an array of `ClearFlag` to determine which buffer is to be cleared.
#[inline]
pub fn clear(clear_flags: &[ClearFlag]) {
    let mut mask = 0;
    clear_flags.iter().for_each(|flag| mask |= flag.bits);
    unsafe { raw_gl::Clear(mask) };
}

/// Specifies the affine transformation of `x` and `y` from normalized device
/// coordinates to window coordinates.
#[inline]
pub fn set_view_port(x: u32, y: u32, width: u32, height: u32) {
    unsafe { raw_gl::Viewport(x as i32, y as i32, width as i32, height as i32) };
}

// TODO: Implement other shader types
pub enum ShaderType {
    Vertex,
    Fragment,
    // Geometry,
    // Compute,
    // TessControl,
    // TessEvaluation,
}

impl ShaderType {
    fn value(&self) -> u32 {
        match *self {
            ShaderType::Vertex => raw_gl::VERTEX_SHADER,
            ShaderType::Fragment => raw_gl::FRAGMENT_SHADER,
            // ShaderType::Geometry => raw_gl::GEOMETRY_SHADER,
            // ShaderType::Compute => raw_gl::COMPUTE_SHADER,
            // ShaderType::TessControl => raw_gl::TESS_CONTROL_SHADER,
            // ShaderType::TessEvaluation => raw_gl::TESS_EVALUATION_SHADER,
        }
    }
}

pub struct ShaderId {
    id: u32,
}

/// Creates an empty shader object and returns its internal id.
///
/// Returns Err if OpenGL fails to create a new shader object.
#[inline]
pub fn create_shader(shader_type: ShaderType) -> Result<ShaderId, GLError> {
    let id = unsafe { raw_gl::CreateShader(shader_type.value()) };
    if id == 0 {
        Err(GLError::CreateShader)
    } else {
        Ok(ShaderId { id })
    }
}

/// Frees the memory and invalidates the id associated with the shader object.
/// If the shader is still attached to a program, it will be marked for
/// deletion and will be deleted after it is detached.
#[inline]
pub fn delete_shader(shader: &ShaderId) {
    unsafe { raw_gl::DeleteShader(shader.id) };
}

/// Sets the source code in the given shader object. The source string is not
/// scanned or parsed at this time; it is simply copied into the specific
/// shader object.
///
/// Returns Err if `source` contains an internal nul byte.
#[inline]
pub fn set_shader_source(shader: &ShaderId, source: &str) -> Result<(), GLError> {
    let source = convert_str_into_c_string(source)?;
    unsafe { raw_gl::ShaderSource(shader.id, 1, &source.as_ptr(), std::ptr::null()) };
    Ok(())
}

/// Converts `str` into a CString.
///
/// Returns Err if `str` contains an internal nul byte.
use std::ffi::CString;
#[inline]
fn convert_str_into_c_string(str: &str) -> Result<CString, GLError> {
    match CString::new(str.as_bytes()) {
        Ok(c_string) => Ok(c_string),
        Err(err) => Err(GLError::NulByteInStr(err)),
    }
}

/// Compiles the source code that has been stored in the shader object using
/// `gl::set_shader_source`.
///
/// Returns Err if compilation fails or OpenGL returns a bad info log on error.
pub fn compile_shader(shader: &ShaderId) -> Result<(), GLError> {
    unsafe { raw_gl::CompileShader(shader.id) };

    let mut success = raw_gl::TRUE as i32;
    unsafe { raw_gl::GetShaderiv(shader.id, raw_gl::COMPILE_STATUS, &mut success) };

    if success == raw_gl::TRUE as i32 {
        Ok(())
    } else {
        let info_log = unsafe { get_info_log(true, shader.id) }?;
        Err(GLError::CompileShader(info_log.to_owned()))
    }
}

pub struct ProgramId {
    id: u32,
}

/// Creates an empty program object and returns its internal id.
///
/// Returns Err if OpenGL fails to create a new program object.
#[inline]
pub fn create_program() -> Result<ProgramId, GLError> {
    let id = unsafe { raw_gl::CreateProgram() };
    if id == 0 {
        Err(GLError::CreateProgram)
    } else {
        Ok(ProgramId { id })
    }
}

/// Frees the memory and invalidates the id associated with the shader object.
/// If the shader is still bound to a render state, it will be marked for
/// deletion and will be deleted after it is unbound.
#[inline]
pub fn delete_program(program: &ProgramId) {
    unsafe { raw_gl::DeleteProgram(program.id) };
}

/// Specifies a shader object to be included when the program is linked.
/// `detach_shader()` should be called after linking.
#[inline]
pub fn attach_shader(program: &ProgramId, shader: &ShaderId) {
    unsafe { raw_gl::AttachShader(program.id, shader.id) };
}

/// Detaches a shader object from a specified program.
/// If the shader has been marked for deletion, it will be deleted too.
#[inline]
pub fn detach_shader(program: &ProgramId, shader: &ShaderId) {
    unsafe { raw_gl::DetachShader(program.id, shader.id) };
}

/// Compiles the source code that has been stored in the shader object using
/// `gl::set_shader_source`.
///
/// Returns Err if compilation fails or OpenGL returns a bad info log on error.
pub fn link_program(program: &ProgramId) -> Result<(), GLError> {
    unsafe { raw_gl::LinkProgram(program.id) };

    let mut success = raw_gl::TRUE as i32;
    unsafe { raw_gl::GetProgramiv(program.id, raw_gl::LINK_STATUS, &mut success) };

    if success == raw_gl::TRUE as i32 {
        Ok(())
    } else {
        let info_log = unsafe { get_info_log(false, program.id) }?;
        Err(GLError::LinkProgram(info_log))
    }
}

/// Gets shader and program info logs. Used if shader compilation or program
/// linking doesn't return `raw_gl::TRUE`.
///
/// `id` must be a valid shader id or a program id.
///
/// Returns Err if OpenGL returns a bad info log.
unsafe fn get_info_log<'a>(is_shader: bool, id: u32) -> Result<String, GLError> {
    let mut len: i32 = 0;
    if is_shader {
        raw_gl::GetShaderiv(id, raw_gl::INFO_LOG_LENGTH, &mut len);
    } else {
        raw_gl::GetProgramiv(id, raw_gl::INFO_LOG_LENGTH, &mut len);
    }
    let len = len as usize;

    let mut info_log: Vec<u8> = Vec::with_capacity(len + 1);
    info_log.extend([b' '].iter().cycle().take(len));

    if is_shader {
        raw_gl::GetShaderInfoLog(
            id,
            len as i32,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
    } else {
        raw_gl::GetProgramInfoLog(
            id,
            len as i32,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
    }

    // match std::str::from_utf8(&info_log) {
    //     Ok(str) => Ok(str),
    //     Err(err) => Err(GLError::Utf8(err)),
    // }
    Ok(std::str::from_utf8(&info_log)?.to_owned())
}

/// Sets the specified program object of the current rendering state.
#[inline]
pub fn bind_program(program: &ProgramId) {
    unsafe { raw_gl::UseProgram(program.id) };
}

/// Invalidates the program object of the current rendering state.
#[inline]
pub fn unbind_program() {
    unsafe { raw_gl::UseProgram(0) };
}

pub struct UniformLocation {
    location: i32,
}

/// Gets the location of a specific uniform variable within a program object.
/// `name` must be an active uniform variable name of `shader_id`.
///
/// Returns Err if `name` does not correspond to an active uniform variable.
#[inline]
pub fn get_uniform_location(
    program_id: &ProgramId,
    name: &str,
) -> Result<UniformLocation, GLError> {
    let c_name = convert_str_into_c_string(name)?;
    let location = unsafe { raw_gl::GetUniformLocation(program_id.id, c_name.as_ptr()) };
    if location == -1 {
        Err(GLError::InvalidUniformName(name.to_owned()))
    } else {
        Ok(UniformLocation { location })
    }
}

/// Modifies the value of a uniform variable of type `vec2`.
#[inline]
pub fn uniform2f(uniform: &UniformLocation, data: &Vector2) {
    unsafe { raw_gl::Uniform2f(uniform.location, data.x, data.y) };
}

/// Modifies the value of a uniform variable of type `vec3`.
#[inline]
pub fn uniform3f(uniform: &UniformLocation, data: &Vector3) {
    unsafe { raw_gl::Uniform3f(uniform.location, data.x, data.y, data.z) };
}

/// Modifies the value of a uniform variable of type `vec4`.
#[inline]
pub fn uniform4f(uniform: &UniformLocation, data: &Vector4) {
    unsafe { raw_gl::Uniform4f(uniform.location, data.x, data.y, data.z, data.w) };
}

pub struct VertexArrayId {
    id: u32,
}

/// Creates an unused vertex array object and returns its internal id.
#[inline]
pub fn create_vertex_array() -> VertexArrayId {
    let mut id: u32 = 0;
    unsafe { raw_gl::GenVertexArrays(1, &mut id) };
    VertexArrayId { id }
}

/// Deletes a vertex array object. Once deleted, it has no contents and it is
/// available for OpenGL to assign a new vao to. If the vao is currently bound,
/// it is automatically unbound and the default vao becomes current.
#[inline]
pub fn delete_vertex_array(vertex_array: &VertexArrayId) {
    unsafe { raw_gl::DeleteVertexArrays(1, [vertex_array.id].as_ptr()) };
}

/// Binds a vertex array object
#[inline]
pub fn bind_vertex_array(vertex_array: &VertexArrayId) {
    unsafe { raw_gl::BindVertexArray(vertex_array.id) };
}

/// Unbinds any currently bound vertex array object
#[inline]
pub fn unbind_vertex_array() {
    unsafe { raw_gl::BindVertexArray(0) };
}

/// Specify the `attrib_location` and data format of the array of vertex
/// attributes specified in the buffer currently bound to the
/// `BufferType::ArrayBuffer` target.
///
/// location: index of the vertex attribute to be modified. [0, 15]
///
/// size: the number of components per vertex attribute. [1, 4]
///
/// normalized: if true, converts data values to [-1, 1] if a signed int or
/// [0, 1] if an unsigned int.
///
/// stride: the offset between consecutive vertex attributes. If 0, the vertex
/// attributes are understood to be tightly packed in the array.
///
/// offset: the offset of the first component of the first vertex attribute in
/// the array.
#[inline]
pub fn vertex_attrib_pointer_float(
    location: usize,
    size: usize,
    normalized: bool,
    stride: usize,
    offset: usize,
) {
    debug_assert!(location < 16);
    debug_assert!(size < 5);
    unsafe {
        raw_gl::VertexAttribPointer(
            location as u32,
            size as i32,
            raw_gl::FLOAT,
            normalized as u8,
            (stride * std::mem::size_of::<f32>()) as i32,
            (offset * std::mem::size_of::<f32>()) as *const raw_gl::types::GLvoid,
        )
    }
}

/// Enable a vertex attribute array
#[inline]
pub fn enable_vertex_attrib(location: usize) {
    debug_assert!(location < 16);
    unsafe { raw_gl::EnableVertexAttribArray(location as u32) };
}

/// Disable a vertex attribute array
#[inline]
pub fn disable_vertex_attrib(location: usize) {
    debug_assert!(location < 16);
    unsafe { raw_gl::DisableVertexAttribArray(location as u32) };
}

#[derive(Debug)]
pub struct BufferId {
    id: u32,
}

/// Creates an unused buffer object and returns its internal id.
#[inline]
pub fn create_buffer() -> BufferId {
    let mut id: u32 = 0;
    unsafe { raw_gl::GenBuffers(1, &mut id) };
    BufferId { id }
}

/// Deletes a buffer object. Once deleted, it has no contents and it is
/// available for OpenGL to assign a new buffer to. If the buffer is currently
/// bound, it is automatically unbound.
#[inline]
pub fn delete_buffer(buffer: &BufferId) {
    unsafe { raw_gl::DeleteBuffers(1, [buffer.id].as_ptr()) };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BufferType {
    ArrayBuffer,
    ElementArrayBuffer,
}

impl BufferType {
    pub fn value(&self) -> u32 {
        match *self {
            BufferType::ArrayBuffer => raw_gl::ARRAY_BUFFER,
            BufferType::ElementArrayBuffer => raw_gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

/// Binds a buffer object
#[inline]
pub fn bind_buffer(buffer_type: BufferType, buffer: &BufferId) {
    unsafe { raw_gl::BindBuffer(buffer_type.value(), buffer.id) };
}

/// Unbinds any currently bound buffer object
#[inline]
pub fn unbind_buffer(buffer_type: BufferType) {
    unsafe { raw_gl::BindBuffer(buffer_type.value(), 0) };
}

// Creates and initializes a buffer object's data store.
#[inline]
pub fn create_static_buffer_data<T>(buffer_type: BufferType, data: &[T]) {
    unsafe {
        raw_gl::BufferData(
            buffer_type.value(),
            (data.len() * std::mem::size_of::<T>()) as isize,
            data.as_ptr() as *const raw_gl::types::GLvoid,
            raw_gl::STATIC_DRAW,
        )
    };
}
