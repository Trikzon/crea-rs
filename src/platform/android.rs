#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::JClass;
use jni::sys::jlong;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "system" fn Java_com_trikzon_omni_1rs_MainActivity_appInit(
    _env: JNIEnv,
    _class: JClass,
    engine_ptr: jlong
) -> jlong {
    let engine = &*(engine_ptr as *mut engine::Engine);
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