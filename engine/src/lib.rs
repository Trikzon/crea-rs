#[cfg_attr(any(target_os="linux", target_os="macos", target_os="windows"), path="platform/desktop.rs")]
#[cfg_attr(target_os="android", path="platform/android.rs")]
pub mod platform;

pub struct Engine<'a> {
    platform: platform::Platform<'a>,
}

impl<'a> Engine<'a> {
    pub fn init(platform: platform::Platform<'a>) -> Self {
        Engine { platform }
    }

    pub fn test(&self, foo: f32) {
        self.platform.test(foo);
    }
}