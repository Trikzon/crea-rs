#![cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]

pub struct Platform<'a> {
    test_callback: fn(f32),
    unused_lifetime: std::marker::PhantomData<&'a f32>
}

impl<'a> Platform<'a> {
    pub fn init(callback: fn(f32)) -> Self {
        Platform {
            test_callback: callback,
            unused_lifetime: std::marker::PhantomData
        }
    }

    pub fn test(&self, foo: f32) {
        (self.test_callback)(foo);
    }
}

pub fn engine_init(callback: fn(f32)) -> crate::Engine<'static> {
    let platform = Platform::init(callback);
    crate::Engine::init(platform)
}
