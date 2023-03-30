use easy_jni::{to_rust, JavaType};
use jni::{
    objects::{JClass, JValue},
    JNIEnv,
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
