#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::{JClass, JObject, GlobalRef};
use jni::sys::{jlong, jfloat};
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

#[no_mangle]
pub unsafe extern "system" fn Java_com_trikzon_omni_1rs_MainActivity_engineInit(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();

    let platform = Platform::init(env, global_ref);
    let engine = crate::Engine::init(platform);

    Box::into_raw(Box::new(engine)) as jlong
}