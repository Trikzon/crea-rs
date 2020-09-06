#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::GlobalRef;
use jni::sys::jfloat;
use jni::JNIEnv;

pub struct Platform<'a> {
    env: JNIEnv<'a>,
    test_callback: GlobalRef
}

impl<'a> Platform<'a> {
    pub fn init(env: JNIEnv<'a>, test_callback: GlobalRef) -> Self {
        Platform { env, test_callback }
    }

    pub fn test(&self, foo: f32) {
        let foo: jfloat = foo;

        self.env.call_method(
            &self.test_callback,
            "testCallback",
            "(F)V",
            &[foo.into()]
        ).unwrap();
    }
}
