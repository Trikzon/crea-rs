use crean::graphics::{Buffer, EventLoop, ShaderProgram, VertexArray, Window};
use crean::maths::{Matrix4, Vector, Vector3};

const SHADER_SOURCE: &str = include_str!("assets/hexagon.glsl");

fn main() {
    // Initialize window and event loop.
    let (mut window, events) = Window::new(512, 512, "Hexagons Are Bestagons");
    let event_loop = EventLoop::new(events);

    window.set_clear_color(0.4, 0.5, 0.7, 1.0);

    // Create data needed for a hexagon.
    let vertices: [f32; 21] = [
        0.0, 0.0, 0.0, // center
        0.0, -0.5, 0.0, // bottom
        0.5, -0.25, 0.0, // bottom right
        0.5, 0.25, 0.0, // top right
        0.0, 0.5, 0.0, // top
        -0.5, 0.25, 0.0, // top left
        -0.5, -0.25, 0.0, // bottom left
    ];
    let vertices1: [f32; 21] = [
        0.0, 0.0, 0.0, // center
        0.0, -0.1, 0.0, // bottom
        0.1, -0.05, 0.0, // bottom right
        0.1, 0.05, 0.0, // top right
        0.0, 0.1, 0.0, // top
        -0.1, 0.05, 0.0, // top left
        -0.1, -0.05, 0.0, // bottom left
    ];
    let colors: [f32; 21] = [
        1.0, 1.0, 1.0, // center
        1.0, 0.0, 0.0, // bottom
        0.0, 1.0, 0.0, // bottom right
        0.0, 0.0, 1.0, // top right
        1.0, 0.0, 0.0, // top
        0.0, 1.0, 0.0, // top left
        0.0, 0.0, 1.0, // bottom left
    ];
    let indices = [0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1];

    use std::rc::Rc;
    // Create OpenGL buffers related to above data.
    let vertices_buffer = Buffer::array(&vertices, 3);
    let vertices_buffer_1 = Buffer::array(&vertices1, 3);
    let colors_buffer = Rc::new(Buffer::array(&colors, 3));
    let indices_buffer = Rc::new(Buffer::indices(&indices));

    // Create OpenGL vertex array and put buffers into it.
    let mut vertex_array = VertexArray::new();
    vertex_array.put_indices_buffer_ref(&indices_buffer);
    vertex_array.put_array_buffer(0, vertices_buffer);
    vertex_array.put_array_buffer_ref(1, &colors_buffer);

    let mut vertex_array1 = VertexArray::new();
    vertex_array1.put_indices_buffer_ref(&indices_buffer);
    vertex_array1.put_array_buffer(0, vertices_buffer_1);
    vertex_array1.put_array_buffer_ref(1, &colors_buffer);

    // Create a shader program.
    let mut shader_program = ShaderProgram::from_multi_shader(SHADER_SOURCE).unwrap();
    shader_program.bind();
    shader_program.define_uniform("uProjection").unwrap();
    // shader_program.define_uniform("uView").unwrap();
    shader_program.define_uniform("uTransformation").unwrap();

    let (width, height) = window.size();
    let projection_matrix = Matrix4::perspective(70.0, width, height, 0.0, 1000.0);
    // let projection_matrix = Matrix4::orthographic(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
    shader_program
        .upload_uniform("uProjection", &projection_matrix)
        .unwrap();
    shader_program.unbind();

    let mut rotation = Vector3::zero();
    let mut rotation1 = Vector3::zero();

    // Game loop
    while !window.should_close() {
        // Check for any OpenGL errors.
        let error = unsafe { raw_gl::GetError() };
        if error != raw_gl::NO_ERROR {
            println!("GL Error: {}", error);
        }

        // Bind needed things for rendering.
        shader_program.bind();
        vertex_array.bind();
        vertex_array.enable_attrib_arrays();

        rotation += Vector3::new(0.0, 0.0, 1.0);
        rotation1 += Vector3::new(0.0, 0.0, -1.0);

        let transformation_matrix =
            Matrix4::transformation(&Vector3::zero(), &rotation, &Vector3::new(1.0, 1.0, 1.0));
        shader_program
            .upload_uniform("uTransformation", &transformation_matrix)
            .unwrap();

        // Render
        unsafe {
            raw_gl::DrawElements(
                raw_gl::TRIANGLES,
                indices.len() as i32,
                raw_gl::UNSIGNED_INT,
                std::ptr::null(),
            )
        };

        // Unbind everything.
        vertex_array.disable_attrib_arrays();
        vertex_array.bind();

        vertex_array1.bind();
        vertex_array1.enable_attrib_arrays();

        let transformation_matrix =
            Matrix4::transformation(&Vector3::zero(), &rotation1, &Vector3::new(1.0, 1.0, 1.0));
        shader_program
            .upload_uniform("uTransformation", &transformation_matrix)
            .unwrap();

        unsafe {
            raw_gl::DrawElements(
                raw_gl::TRIANGLES,
                indices.len() as i32,
                raw_gl::UNSIGNED_INT,
                std::ptr::null(),
            )
        };

        vertex_array1.unbind();
        vertex_array1.disable_attrib_arrays();

        shader_program.unbind();

        // Update the window.
        window.update();
        event_loop.process_events(&mut window);
    }
}
