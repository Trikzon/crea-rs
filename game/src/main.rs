extern crate crean;

use crean::window;

fn main() {
    let mut window = window::Window::new("Title", 1280, 720);

    loop {
        if window.poll_events() {
            break;
        }

        window.update();
    }
}
