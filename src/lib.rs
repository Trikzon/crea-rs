extern crate engine;

#[cfg_attr(target_os="android", path = "platform/android.rs")]
#[cfg_attr(any(target_os="linux", target_os="macos", target_os="windows"), path="platform/desktop.rs")]
pub mod platform;

pub struct App<'a> {
    //TODO: I want App to own Engine, not borrow it
    engine: &'a engine::Engine<'a>,
    foo: f32,
}

impl<'a> App<'a> {
    pub fn init(engine: &'a engine::Engine<'a>) -> Self {
        App { engine, foo: 0.0 }
    }

    pub fn update(&mut self) {
        self.foo = self.foo + 0.1;
        self.engine.test(self.foo);
    }
}