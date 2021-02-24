use crate::graphics::{gl, Buffer};
use std::rc::Rc;

pub struct VertexArray {
    indices_buffer: Option<Rc<Buffer>>,
    array_buffers: [Option<Rc<Buffer>>; 16],

    vertex_array_id: gl::VertexArrayId,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vec = Vec::<Option<Rc<Buffer>>>::with_capacity(16);
        for _ in 0..16 {
            vec.push(None);
        }

        use std::convert::TryInto;
        let empty_array_buffers = vec.try_into().unwrap();

        VertexArray {
            indices_buffer: None,
            array_buffers: empty_array_buffers,
            vertex_array_id: gl::create_vertex_array(),
        }
    }

    pub fn put_indices_buffer(&mut self, buffer: Buffer) {
        self.put_indices_buffer_ref(&Rc::new(buffer));
    }

    pub fn put_indices_buffer_ref(&mut self, buffer: &Rc<Buffer>) {
        debug_assert_eq!(buffer.buffer_type(), gl::BufferType::ElementArrayBuffer);

        self.bind();
        buffer.bind();
        self.unbind();
        buffer.unbind();

        self.indices_buffer = Some(buffer.clone());
    }

    pub fn put_array_buffer(&mut self, location: usize, buffer: Buffer) {
        self.put_array_buffer_ref(location, &Rc::new(buffer));
    }

    pub fn put_array_buffer_ref(&mut self, location: usize, buffer: &Rc<Buffer>) {
        debug_assert_eq!(buffer.buffer_type(), gl::BufferType::ArrayBuffer);
        debug_assert!(location < 16);

        self.array_buffers[location] = None;

        self.bind();
        buffer.bind();
        gl::enable_vertex_attrib(location);
        gl::vertex_attrib_pointer_float(location, buffer.size(), false, 0, 0);
        gl::disable_vertex_attrib(location);
        self.unbind();
        buffer.unbind();

        self.array_buffers[location] = Some(buffer.clone());
    }

    pub fn enable_attrib_arrays(&self) {
        for (i, buffer) in self.array_buffers.iter().enumerate() {
            if buffer.is_some() {
                gl::enable_vertex_attrib(i);
            }
        }
    }

    pub fn disable_attrib_arrays(&self) {
        for (i, buffer) in self.array_buffers.iter().enumerate() {
            if buffer.is_some() {
                gl::disable_vertex_attrib(i);
            }
        }
    }

    pub fn bind(&self) {
        gl::bind_vertex_array(&self.vertex_array_id);
    }

    pub fn unbind(&self) {
        gl::unbind_vertex_array();
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        gl::delete_vertex_array(&self.vertex_array_id);
    }
}
