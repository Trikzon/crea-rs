extern crate engine;

#[cfg_attr(target_os="android", path = "platform/android.rs")]
pub mod platform;

pub struct App<'a> {
    engine: engine::Engine<'a>,
    foo: f32,
}

impl<'a> App<'a> {
    pub fn init(engine: engine::Engine<'a>) -> Self {
        App { engine, foo: 0.0 }
    }

    pub fn update(&mut self) {
        self.foo = self.foo + 0.1;
        self.engine.test(self.foo);
    }
}