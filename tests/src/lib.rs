use jni::{objects::JClass, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_RustLibrary_easyJni<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
}
