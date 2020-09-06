#[cfg_attr(any(target_os="linux", target_os="macos", target_os="windows"), path="platform/desktop.rs")]
pub mod platform;

pub struct Engine {
    platform: platform::Platform,
}

impl Engine {
    pub fn init(platform: platform::Platform) -> Self {
        Engine { platform }
    }

    pub fn test(&self, foo: f32) {
        self.platform.test(foo);
    }
}