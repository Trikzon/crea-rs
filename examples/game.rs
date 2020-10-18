extern crate crean;

struct Game {
    counter: u64,
}

impl crean::App for Game {
    fn init(&mut self) {
        println!("init");
    }

    fn input(&mut self) {
        println!("input");

    }

    fn update(&mut self) {
        self.counter += 1;
        println!("update");
    }

    fn render(&mut self) {
        println!("render: {}", self.counter);
    }
}

fn main() {
    crean::run(1280, 720, "Crean Engine", &mut Game { counter: 0 });
}
