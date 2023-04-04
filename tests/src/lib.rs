use easy_jni::{
    jni::{
        objects::{JClass, JObject, JString, JValue, JValueOwned},
        strings::JNIString,
        sys::{jarray, jint, jobject, jstring},
        JNIEnv,
    },
    to_rust, Class, JavaArray, JavaType, JavaTypeSignature,
};

#[no_mangle]
pub extern "system" fn Java_RustLibrary_nativeAssertions<'local>(
    mut env: JNIEnv<'local>,
    java_class: &JClass<'local>,
) {
    {
        let byte_data = 4i8;

        let outcome = to_rust(&mut env, java_class, JValue::from(byte_data));

        assert_eq!(Ok(JavaType::Byte(byte_data as u8)), outcome);
    }

    {
        let byte_data = -4i8;

        let outcome = to_rust(&mut env, java_class, JValue::from(byte_data));

        assert_eq!(
            Err(easy_jni::EasyJniError::ByteLessThanZeroNotSupported),
            outcome
        );
    }

    {
        let short_data = 50i16;

        let outcome = to_rust(&mut env, java_class, JValue::from(short_data));

        assert_eq!(Ok(JavaType::Short(short_data)), outcome);
    }

    {
        let int_data = 50i32;

        let outcome = to_rust(&mut env, java_class, JValue::from(int_data));

        assert_eq!(Ok(JavaType::Int(int_data)), outcome);
    }

    {
        let long_data = 50i64;

        let outcome = to_rust(&mut env, java_class, JValue::from(long_data));

        assert_eq!(Ok(JavaType::Long(long_data)), outcome);
    }

    {
        let float_data = 504.99f32;

        let outcome = to_rust(&mut env, &java_class, JValue::from(float_data));

        assert_eq!(Ok(JavaType::Float(float_data)), outcome);
    }

    {
        let double_data = 504.99f64;

        let outcome = to_rust(&mut env, java_class, JValue::from(double_data));

        assert_eq!(Ok(JavaType::Double(double_data)), outcome);
    }

    {
        let bool_data = true;

        let outcome = to_rust(&mut env, java_class, JValue::from(bool_data));

        assert_eq!(Ok(JavaType::Boolean(bool_data)), outcome);
    }

    {
        let void_data = ();

        let outcome = to_rust(&mut env, java_class, JValue::from(void_data));

        assert_eq!(Ok(JavaType::Void), outcome);
    }

    {
        let string_data = String::from("MY_TEST_STRING");
        //let to_jvalue = JavaType::new_jvalue_string(env, java_class, &string_data).unwrap();

        let string = env.new_string(&string_data).unwrap();

        let outcome = to_rust(&mut env, java_class, JValue::from(&string));

        assert_eq!(Ok(JavaType::String(string_data)), outcome);
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyClass<'local>(
    mut env: JNIEnv<'local>,
    java_class: JClass<'local>,
) -> jobject {
    let class_name = "RustyClass";
    let field_name = "message";
    let field_contents = "RUSTY_JNI_CLASS";

    let object = Class::new()
        .add_name(class_name)
        .create(&mut env, &java_class)
        .unwrap();

    let field_signature = JavaTypeSignature::String.bytecode_signature();

    let field_name = JNIString::from(field_name);
    let message = env.new_string(field_contents).unwrap();

    env.set_field(
        &object,
        field_name,
        field_signature,
        JValueOwned::Object(JObject::from(message)).borrow(),
    )
    .unwrap();

    object.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyArray<'local>(
    mut env: JNIEnv<'local>,
    java_class: &'local JClass<'local>,
) -> jarray {
    let create_array =
        JavaArray::create(&mut env, &java_class, JavaTypeSignature::String, 3).unwrap();

    let strings = ["ONE", "TWO", "THREE"];
    for (i, value) in strings.iter().enumerate() {
        let jstring = env.new_string(value).unwrap();
        env.set_object_array_element(&create_array, i as jint, &jstring)
            .unwrap();
    }

    create_array.as_raw()
}

#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyArrayInts<'local>(
    mut env: JNIEnv<'local>,
    java_class: &'local JClass<'local>,
) -> jarray {
    let create_array = JavaArray::create(&mut env, &java_class, JavaTypeSignature::Int, 3).unwrap();

    let strings = [0i32, 1, 2];
    for (i, value) in strings.iter().enumerate() {
        let integer_class = env
            .find_class("java/lang/Integer")
            .expect("Failed to find java/lang/Integer");

        let java_value = env
            .new_object(integer_class, "(I)V", &[JValue::Int(*value)])
            .unwrap();
        env.set_object_array_element(&create_array, i as jint, JObject::from(java_value))
            .unwrap();
    }

    create_array.as_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_resultOfArrayString<'local>(
    mut env: JNIEnv<'local>,
    java_class: JClass<'local>,
) -> jobject {
    let class_name = "ResultOfArrayString";
    let success_field_name = "successData";
    let failure_field_name = "failureData";

    let success_array =
        JavaArray::create(&mut env, &java_class, JavaTypeSignature::String, 3).unwrap();

    {
        let strings = ["ONE", "TWO", "THREE"];
        for (i, value) in strings.iter().enumerate() {
            let jstring = env.new_string(value).unwrap();
            env.set_object_array_element(&success_array, i as jint, &jstring)
                .unwrap();
        }
    }
    let failure_array =
        JavaArray::create(&mut env, &java_class, JavaTypeSignature::String, 0).unwrap();

    let object = Class::new()
        .add_name(class_name)
        .create(&mut env, &java_class)
        .unwrap();
    let field_signature = JavaTypeSignature::String.java_class_name_array().unwrap();

    env.set_field(
        &object,
        success_field_name,
        &field_signature,
        JValueOwned::Object(JObject::from(success_array)).borrow(),
    )
    .unwrap();

    env.set_field(
        &object,
        failure_field_name,
        &field_signature,
        JValueOwned::Object(JObject::from(failure_array)).borrow(),
    )
    .unwrap();

    object.into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_sillyDebugger<'local>(
    mut env: JNIEnv<'local>,
    java_class: JClass<'local>,
    _input: JString<'local>,
) -> jstring {
    let class_name = "ResultOfArrayString";
    let success_field_name = "successData";

    let success_array =
        JavaArray::create(&mut env, &java_class, JavaTypeSignature::String, 3).unwrap();

    let object = Class::new()
        .add_name(class_name)
        .create(&mut env, &java_class)
        .unwrap();

    let field_signature = JavaTypeSignature::String.java_class_name_array().unwrap();

    match env.set_field(
        &object,
        success_field_name,
        field_signature,
        JValueOwned::Object(JObject::from(success_array)).borrow(),
    ) {
        Ok(_) => env.new_string("DEBUG_TRUE").unwrap().into_raw(),
        Err(error) => env.new_string(error.to_string()).unwrap().into_raw(),
    }
}
