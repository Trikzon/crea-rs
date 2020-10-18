use glfw::{Context, Glfw, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window {
    width: i32,
    height: i32,
    title: String,

    // gl and glfw variables
    glfw: Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    gl: gl::Gl,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
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
        unsafe {
            gl.Viewport(0, 0, width, height);
            gl.ClearColor(1.0, 0.0, 0.0, 1.0);
        }

        Window {
            width, height, title,
            window, glfw, events, gl,
        }
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();

        self.glfw.poll_events();
        if self.process_events() {
            self.close();
        }

        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn process_events(&mut self) -> bool {
        use glfw::{Key, Action};
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    return true;
                },
                _ => {}
            }
        }
        false
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }
}
