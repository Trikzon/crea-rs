extern crate gl;
extern crate sdl2;

mod render;
mod resources;

use resources::Resources;
use render::data;
use render::buffer;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    color: data::f32_f32_f32,
}

impl Vertex {
    fn vertex_attrib_pointers(gl: &gl::GL) {
        // byte offset between consecutive attributes
        let stride = std::mem::size_of::<Self>();

        let location = 0; // layout (location = 0)
        let offset = 0; // offset of the first component

        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset);
        }

        let location = 1; // layout (location = 1)
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>(); // offset of the first component

        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset);
        }
    }
}

fn main() {
    let res = Resources::from_relative_exe_path("assets").unwrap();

    let mut display = render::Display::create(
        "minesweeper-rs", 1280, 720
    );
    let gl = display.gl().clone();

    let shader = render::Shader::load_shaders(
        &gl, &res,
        &["shaders/triangle.frag", "shaders/triangle.vert"]
    ).unwrap();

    let vertices: Vec<Vertex> = vec![
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            color: (1.0, 0.0, 0.0).into(),
        },
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            color: (0.0, 1.0, 0.0).into(),
        },
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            color: (0.0, 0.0, 1.0).into(),
        },
    ];

    let vbo = buffer::ArrayBuffer::new(&gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    // set up vertex array object
    let vao = buffer::VertexArray::new(&gl);
    vao.bind();
    vbo.bind();
    Vertex::vertex_attrib_pointers(&gl);
    vbo.unbind();
    vao.unbind();

    loop {
        if display.poll_events() {
            break;
        }

        // input
        // logic
        // render
        shader.bind();
        vao.bind();
        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3, // number of indices to be rendered
            )
        }


        display.update();
    }
}
