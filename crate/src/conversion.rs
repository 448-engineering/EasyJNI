use crate::{EasyJNIResult, EasyJniError, JavaType};
use jni::{
    self,
    objects::{JClass, JObject, JString, JValue, JValueGen},
    JNIEnv,
};

pub fn to_rust<'local>(
    env: &mut JNIEnv<'local>,
    // Static class which owns this method.
    _class: &JClass<'local>,
    input: JValue,
) -> EasyJNIResult<JavaType> {
    let outcome = match input {
        JValueGen::Byte(value) => {
            if value < 0 {
                return Err(crate::EasyJniError::ByteLessThanZeroNotSupported);
            } else {
                JavaType::Byte(value as u8)
            }
        }
        JValueGen::Double(value) => JavaType::Double(value),
        JValueGen::Float(value) => JavaType::Float(value),
        JValueGen::Int(value) => JavaType::Int(value),
        JValueGen::Long(value) => JavaType::Long(value),
        JValueGen::Short(value) => JavaType::Short(value),
        JValueGen::Bool(value) => {
            if value == 0u8 {
                JavaType::Boolean(false)
            } else if value == 1u8 {
                JavaType::Boolean(true)
            } else {
                JavaType::Boolean(false)
            }
        }
        JValueGen::Void => JavaType::Void,
        JValueGen::Object(value) => {
            let value = unsafe { JObject::from_raw(value.as_raw()) };

            if let Ok(inner_value) = JString::try_from(value) {
                let string_outcome = env.get_string(&inner_value)?.into();

                return Ok(JavaType::String(string_outcome));
            } else {
                return Err(EasyJniError::UnsupportedJavaType);
            }
        }
        _ => return Err(EasyJniError::UnsupportedJavaType),
    };

    Ok(outcome)
}
