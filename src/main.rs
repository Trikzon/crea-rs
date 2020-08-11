extern crate gl;
extern crate sdl2;

mod render;

fn main() {
    let mut display = render::Display::create(
        "minesweeper-rs", 1280, 720
    );
    let gl = display.gl();

    loop {
        if display.poll_events() {
            break;
        }

        // input
        // logic
        // render
        display.update();
    }
}
