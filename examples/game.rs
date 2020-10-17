extern crate crean;

fn main() {
    let mut window = crean::Window::new("Crean", 1280, 720);

    loop {
        if window.poll_events() {
            break;
        }

        window.update();
    }
}
