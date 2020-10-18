extern crate gl;
extern crate glfw;

mod window;

pub fn run(width: u32, height: u32, title: &str, app: &mut impl App) {
    let (mut window, events) = window::Window::new(width, height, title);

    app.init();

    while !window.should_close() {
        window.update();

        use glfw::{Action, Key, WindowEvent};
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.close();
                },
                WindowEvent::FramebufferSize(width, height) => {
                    window.resize(width, height);
                },
                _ => {}
            }
        }
        app.input();
        app.update();
        app.render();
    }
}

pub trait App {
    fn init(&mut self);
    fn input(&mut self);
    fn update(&mut self);
    fn render(&mut self);
}
