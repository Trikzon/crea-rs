extern crate crean;
extern crate gl;

use crean::Crean;
use std::{ptr, mem};
use std::os::raw::c_void;
use gl::types::*;

const VERTEX_ARRAY: [f32; 28] = [
    // position      // color
    0.5, -0.5, 0.0,  1.0, 0.0, 0.0, 1.0, // bottom right 0
    -0.5, 0.5, 0.0,  0.0, 1.0, 0.0, 1.0, // top left     1
    0.5, 0.5, 0.0,   0.0, 0.0, 1.0, 1.0, // top right    2
    -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 1.0, // bottom left  3
];

// IMPORTANT: Must be in counter-clockwise order
const ELEMENT_ARRAY: [u32; 6] = [
    2, 1, 0,
    0, 1, 3,
];

struct Game {
    shader: Option<crean::Shader>,
    vao_id: u32,
    vbo_id: u32,
    ebo_id: u32,
}

impl crean::App for Game {
    fn init(&mut self, crean: &mut Crean) {
        self.shader = Some(crean::Shader::new(
            crean.window().gl(), "./assets/shaders/default.glsl"
        ).unwrap());

        unsafe {
            // Generate VAO, VBO, and EBO buffer objects, and send to GPU
            crean.window().gl().GenVertexArrays(1, &mut self.vao_id);
            crean.window().gl().BindVertexArray(self.vao_id);
            crean.window().gl().GenBuffers(1, &mut self.vbo_id);
            crean.window().gl().BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);

            // upload the vertices
            crean.window().gl().BufferData(gl::ARRAY_BUFFER,
               (VERTEX_ARRAY.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
               &VERTEX_ARRAY[0] as *const f32 as *const c_void,
               gl::STATIC_DRAW
            );

            crean.window().gl().GenBuffers(1, &mut self.ebo_id);
            crean.window().gl().BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
            crean.window().gl().BufferData(gl::ELEMENT_ARRAY_BUFFER,
               (ELEMENT_ARRAY.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
               &ELEMENT_ARRAY[0] as *const u32 as *const c_void,
               gl::STATIC_DRAW
            );

            // Add the vertex attribute pointers
            let position_size = 3;
            let color_size = 4;
            let float_size_bytes = mem::size_of::<f32>() as i32;
            let vertex_size_bytes = (position_size + color_size) * float_size_bytes;
            crean.window().gl().VertexAttribPointer(
                0, position_size, gl::FLOAT, gl::FALSE, vertex_size_bytes, ptr::null()
            );
            crean.window().gl().EnableVertexAttribArray(0);

            crean.window().gl().VertexAttribPointer(
                1, color_size, gl::FLOAT, gl::FALSE, vertex_size_bytes,
                (position_size * float_size_bytes) as *const c_void
            );
        }
    }

    fn input(&mut self, crean: &mut Crean) {
        let input = crean.input();
        use crean::Key;
        if input.is_key_just_pressed(Key::R) {
            println!("Pressed R");
        }
        if input.is_key_pressed(Key::R) {
            println!("Holding R");
        }
    }

    fn update(&mut self, _crean: &mut Crean, _dt: f64) {
    }

    fn render(&mut self, crean: &mut Crean) {
        if let Some(shader) = &self.shader {
            shader.enable();
        }
        unsafe {
            // Bind shader program
            crean.window().gl().BindVertexArray(self.vao_id);

            // Enable the vert attribute pointers
            crean.window().gl().EnableVertexAttribArray(0);
            crean.window().gl().EnableVertexAttribArray(1);

            crean.window().gl().DrawElements(
                gl::TRIANGLES, ELEMENT_ARRAY.len() as i32, gl::UNSIGNED_INT, ptr::null()
            );

            // Unbind everything
            crean.window().gl().DisableVertexAttribArray(1);
            crean.window().gl().DisableVertexAttribArray(0);
            crean.window().gl().BindVertexArray(0);
            crean.window().gl().UseProgram(0);
        }
    }
}

fn main() {
    crean::run(1280, 720, "Crean Engine", &mut Game {
        shader: None,
        vao_id: 0,
        vbo_id: 0,
        ebo_id: 0,
    });
}
