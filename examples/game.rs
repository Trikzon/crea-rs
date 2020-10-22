extern crate crean;
extern crate gl;

use crean::Crean;
use std::ffi::CString;
use std::{str, ptr, mem};
use std::os::raw::c_void;
use gl::types::*;

const VERT_SHADER_SOURCE: &str = "
    #version 330 core
    layout (location=0) in vec3 aPos;
    layout (location=1) in vec4 aColor;
    out vec4 fColor;
    void main() {
        fColor = aColor;
        gl_Position = vec4(aPos, 1.0);
    }";

const FRAG_SHADER_SOURCE: &str = "
    #version 330 core

    in vec4 fColor;

    out vec4 color;

    void main() {
        color = fColor;
    }";

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
    vert_id: u32,
    frag_id: u32,
    program_id: u32,
    vao_id: u32,
    vbo_id: u32,
    ebo_id: u32,
}

impl crean::App for Game {
    fn init(&mut self, crean: &mut Crean) {
        // Compile and link shaders

        unsafe {
            // Frist load and compile the vert shader
            self.vert_id = crean.window().gl().CreateShader(gl::VERTEX_SHADER);
            // Pass the shader source to the GPU
            let c_str_vert = CString::new(VERT_SHADER_SOURCE.as_bytes()).unwrap();
            crean.window().gl().ShaderSource(self.vert_id, 1, &c_str_vert.as_ptr(), ptr::null());
            crean.window().gl().CompileShader(self.vert_id);

            // Check for errors in compilation
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            crean.window().gl().GetShaderiv(self.vert_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                crean.window().gl().GetShaderInfoLog(self.vert_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            // Second load and compile the frag shader
            self.frag_id = crean.window().gl().CreateShader(gl::FRAGMENT_SHADER);
            // Pass the shader source to the GPU
            let c_str_vert = CString::new(FRAG_SHADER_SOURCE.as_bytes()).unwrap();
            crean.window().gl().ShaderSource(self.frag_id, 1, &c_str_vert.as_ptr(), ptr::null());
            crean.window().gl().CompileShader(self.frag_id);

            // Check for errors in compilation
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            crean.window().gl().GetShaderiv(self.frag_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                crean.window().gl().GetShaderInfoLog(self.frag_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            // Link shaders and check for errors
            self.program_id = crean.window().gl().CreateProgram();
            crean.window().gl().AttachShader(self.program_id, self.vert_id);
            crean.window().gl().AttachShader(self.program_id, self.frag_id);
            crean.window().gl().LinkProgram(self.program_id);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            crean.window().gl().GetProgramiv(self.program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                crean.window().gl().GetProgramInfoLog(self.program_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

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

    fn update(&mut self, _crean: &mut Crean, dt: f64) {
        println!("dt: {}", dt);
    }

    fn render(&mut self, crean: &mut Crean) {
        unsafe {
            // Bind shader program
            crean.window().gl().UseProgram(self.program_id);
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
        vert_id: 0,
        frag_id: 0,
        program_id: 0,
        vao_id: 0,
        vbo_id: 0,
        ebo_id: 0,
    });
}
