extern crate crean;

fn main() {
    let mut window = crean::Window::new(1280, 720, "Crean Engine");

    loop {
        if window.should_close() {
            break;
        }

        window.update();
    }
}
