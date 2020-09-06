pub struct Platform {
    test_callback: fn(f32),
}

impl Platform {
    pub fn init(callback: fn(f32)) -> Self {
        Platform { test_callback: callback }
    }

    pub fn test(&self, foo: f32) {
        (self.test_callback)(foo);
    }
}