use crate::graphics::gl;
pub use crate::graphics::gl::BufferType;

#[derive(Debug)]
pub struct Buffer {
    len: usize,
    size: usize,

    buffer_id: gl::BufferId,
    buffer_type: BufferType,
}

impl Buffer {
    pub fn indices(data: &[u32]) -> Self {
        Buffer::from_array(BufferType::ElementArrayBuffer, data, 0)
    }

    pub fn array(data: &[f32], size: usize) -> Self {
        Buffer::from_array(BufferType::ArrayBuffer, data, size)
    }

    fn from_array<T>(buffer_type: BufferType, data: &[T], size: usize) -> Self {
        let buffer = Buffer {
            len: data.len(),
            size,
            buffer_id: gl::create_buffer(),
            buffer_type,
        };

        buffer.bind();
        gl::create_static_buffer_data(buffer.buffer_type, &data);
        buffer.bind();

        buffer
    }

    pub fn bind(&self) {
        gl::bind_buffer(self.buffer_type, &self.buffer_id);
    }

    pub fn unbind(&self) {
        gl::unbind_buffer(self.buffer_type);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn buffer_type(&self) -> BufferType {
        self.buffer_type
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        gl::delete_buffer(&self.buffer_id);
    }
}
