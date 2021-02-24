use crate::graphics::gl;
use glfw::{Context, WindowEvent, WindowMode};

pub struct Window {
    width: u32,
    height: u32,
    title: String,

    glfw: glfw::Glfw,
    glfw_window: glfw::Window,
}

impl Window {
    pub fn new<T>(width: u32, height: u32, title: T) -> (Self, Receiver<(f64, WindowEvent)>)
    where
        T: Into<String>,
    {
        let title = title.into();

        let nop: Option<glfw::ErrorCallback<()>> = None;
        let mut glfw = glfw::init(nop).unwrap();

        use glfw::WindowHint;
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut glfw_window, events) = glfw
            .create_window(width, height, &title, WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw_window.make_current();
        gl::load_with(|s| glfw_window.get_proc_address(s));

        glfw_window.set_key_polling(true);
        glfw_window.set_framebuffer_size_polling(true);

        let mut window = Window {
            width,
            height,
            title,
            glfw,
            glfw_window,
        };
        window.resize(width, height);

        (window, events)
    }

    pub fn update(&mut self) {
        self.glfw_window.swap_buffers();

        use crate::graphics::gl::ClearFlag;
        gl::clear(&[ClearFlag::COLOR_BUFFER, ClearFlag::DEPTH_BUFFER]);
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        gl::set_view_port(0, 0, width, height);
    }

    #[inline]
    pub fn resize_i(&mut self, width: i32, height: i32) {
        assert!(width >= 0);
        assert!(height >= 0);
        self.resize(width as u32, height as u32);
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    pub fn title(&self) -> &'_ str {
        &self.title
    }

    #[inline]
    pub fn set_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        gl::set_clear_color(red, green, blue, alpha);
    }

    #[inline]
    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }
}

use std::sync::mpsc::Receiver;

pub struct EventLoop {
    events: Receiver<(f64, WindowEvent)>,
}

impl EventLoop {
    pub fn new(events: Receiver<(f64, WindowEvent)>) -> Self {
        EventLoop { events }
    }

    pub fn process_events(&self, window: &mut Window) {
        window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    println!("test");
                }
                WindowEvent::FramebufferSize(w, h) => {
                    window.resize_i(w, h);
                }
                _ => {}
            }
        }
    }
}
