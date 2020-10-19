extern crate gl;
extern crate glfw;

mod window;
mod input;

pub use glfw::Key;
pub use glfw::MouseButton;
pub use input::Input;
pub use window::Window;

pub struct Crean {
    window: window::Window,
    input: input::Input,
}

impl Crean {
    pub fn window(&mut self) -> &mut window::Window {
        &mut self.window
    }

    pub fn input(&mut self) -> &mut input::Input {
        &mut self.input
    }
}

pub fn run(width: u32, height: u32, title: &str, app: &mut impl App) {
    let (window, events) = window::Window::new(width, height, title);
    let input = input::Input::new();
    let mut crean = Crean { window, input };

    app.init(&mut crean);

    while !crean.window().should_close() {
        crean.window().update();

        use glfw::WindowEvent;
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    crean.window().resize(width, height);
                },
                WindowEvent::CursorPos(x, y) => {
                    crean.input.cursor_pos_event(x, y);
                },
                WindowEvent::MouseButton(button, action, modifiers) => {
                    crean.input.mouse_button_event(button, action, modifiers);
                },
                WindowEvent::Key(key, _scan_code, action, modifiers) => {
                    crean.input.key_event(key, action, modifiers);
                },
                _ => {}
            }
        }
        let dt = crean.window.get_delta_time();

        app.input(&mut crean);
        app.update(&mut crean, dt);
        app.render(&mut crean);

        crean.input.end_frame();
    }
}

pub trait App {
    fn init(&mut self, crean: &mut Crean);
    fn input(&mut self, crean: &mut Crean);
    fn update(&mut self, crean: &mut Crean, dt: f64);
    fn render(&mut self, crean: &mut Crean);
}
