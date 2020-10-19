extern crate crean;

use crean::Crean;

struct Game {
    counter: u64,
}

impl crean::App for Game {
    fn init(&mut self, crean: &mut Crean) {
    }

    fn input(&mut self, crean: &mut Crean) {
        let input = crean.input();
        use crean::Key;
        if input.is_key_just_pressed(Key::R) {
            println!("Pressed R");
        }
        if input.is_key_pressed(Key::R) {
            println!("Holding R");
        }
    }

    fn update(&mut self, crean: &mut Crean) {
        self.counter += 1;
    }

    fn render(&mut self, crean: &mut Crean) {
    }
}

fn main() {
    crean::run(1280, 720, "Crean Engine", &mut Game { counter: 0 });
}
