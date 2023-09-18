use easy_jni::{
    jni::{
        objects::{JClass, JObject, JValueOwned},
        sys::{jint, jobject},
        JNIEnv,
    },
    Class, JavaArray, JavaTypeSignature,
};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_dirReaderWithResult<'local>(
    mut env: JNIEnv<'local>,
    java_class: JClass<'local>,
) -> jobject {
    let class_name = "DirReaderWithResult";
    let success_field_name = "successData";
    let failure_field_name = "failureData";

    let dirs = smol::block_on(async { read_dir().await.expect("Could not cread dir via smol") });

    let success_array =
        JavaArray::create(&mut env, &java_class, JavaTypeSignature::String, dirs.len())
            .expect("COULD NOT CREATE ARRAY");

    {
        for (i, value) in dirs.iter().enumerate() {
            let jstring = env.new_string(value).unwrap();
            env.set_object_array_element(&success_array, i as jint, &jstring)
                .expect("Could not set object array element");
        }
    }
    let failure_array = JavaArray::create(&mut env, &java_class, JavaTypeSignature::String, 0)
        .expect("Could not create java array");

    let object = Class::new(class_name)
        .create(&mut env, &java_class)
        .expect("Coud not create class to init");
    let field_signature = JavaTypeSignature::String.java_class_name_array().unwrap();

    env.set_field(
        &object,
        success_field_name,
        &field_signature,
        JValueOwned::Object(JObject::from(success_array)).borrow(),
    )
    .expect("Could not set class field `success` after init");

    env.set_field(
        &object,
        failure_field_name,
        &field_signature,
        JValueOwned::Object(JObject::from(failure_array)).borrow(),
    )
    .unwrap();

    object.into_raw()
}

pub async fn read_dir() -> Result<Vec<String>, smol::io::Error> {
    use futures_lite::stream::StreamExt;
    use smol::fs::read_dir;

    let mut dir = read_dir(".").await?;

    let mut dirs = Vec::<String>::new();

    while let Some(entry) = dir.try_next().await? {
        println!("RUST DIR ENTRY- {:?}", &entry.path());

        dirs.push(entry.path().to_string_lossy().to_string())
    }

    Ok(dirs)
}
