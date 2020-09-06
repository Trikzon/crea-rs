#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "system" fn Java_com_trikzon_omni_1rs_MainActivity_appInit(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();

    let platform = engine::platform::Platform::init(env, global_ref);
    let engine = engine::Engine::init(platform);
    let app = crate::App::init(engine);

    Box::into_raw(Box::new(app)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_trikzon_omni_1rs_MainActivity_appUpdate(
    _env: JNIEnv,
    _class: JClass,
    app_ptr: jlong
) {
    let app = &mut *(app_ptr as *mut crate::App);
    app.update();
}