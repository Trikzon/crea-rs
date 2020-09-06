extern crate engine;

pub struct App {
    engine: engine::Engine,
    foo: f32,
}

impl App {
    pub fn init(engine: engine::Engine) -> Self {
        App { engine, foo: 0.0 }
    }

    pub fn update(&mut self) {
        self.foo = self.foo + 0.1;
        self.engine.test(self.foo);
    }
}