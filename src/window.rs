use glfw::{Context, Glfw, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window {
    width: i32,
    height: i32,
    title: String,

    // gl and glfw variables
    glfw: Glfw,
    window: glfw::Window,
    gl: gl::Gl,

    // used for fps and delta time calculation
    last_time: f64,
}

impl Window {
    pub(crate) fn new(width: u32, height: u32, title: &str) -> (Self, Receiver<(f64, WindowEvent)>) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
            .expect("Failed to initialize GLFW");

        use glfw::WindowHint;
        glfw.default_window_hints();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(
            width, height, title,
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // V-Sync
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);

        let width = width as i32;
        let height = height as i32;
        let title = title.to_string();
        let last_time = glfw.get_time();

        let window = Window { width, height, title, glfw, window, gl, last_time };
        window.resize(width, height);
        window.set_clear_color(1.0, 0.0, 0.0);

        (window, events)
    }

    pub(crate) fn update(&mut self) {
        self.window.swap_buffers();

        self.glfw.poll_events();

        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }

    pub fn set_clear_color(&self, red: f32, green: f32, blue: f32) {
        unsafe {
            self.gl.ClearColor(red, green, blue, 1.0);
        }
    }

    pub fn get_delta_time(&mut self) -> f64 {
        let time = self.glfw.get_time();
        let dt = time - self.last_time;
        self.last_time = time;

        dt
    }
}
