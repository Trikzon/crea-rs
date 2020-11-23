extern crate gl;
extern crate glfw;

mod window;
mod input;
mod shader;

pub use glfw::Key;
pub use glfw::MouseButton;
pub use input::Input;
pub use window::Window;
pub use shader::Shader;

/// A struct to house underlying engine structs like `window::Window` and `input::Input`.
/// This gets passed to the application's run-loop methods to give access to engine features.
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

/// Crean's entrypoint function. It creates the `Crean` struct and starts the app run-loop.
/// This function only returns when the app closes so it should be called after any pre-init code
/// has been run.
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

/// Implement on the main app struct. Provides methods that are called by the engine and controls
/// the run-loop.
pub trait App {
    /// Called once after the engine has been initiated.
    /// Resource loading should take place at this stage.
    fn init(&mut self, crean: &mut Crean);
    /// Runs every tick. Input from mouse/keyboard should take place at this stage.
    fn input(&mut self, crean: &mut Crean);
    /// Runs every tick. Physics and other update-like behavior should happen at this stage.
    /// Provides a delta-time variable for use in physics so fps and physics aren't linked.
    fn update(&mut self, crean: &mut Crean, dt: f64);
    /// Runs every tick. Render calls should take place at this stage.
    fn render(&mut self, crean: &mut Crean);
}
