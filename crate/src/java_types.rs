use crate::{EasyJNIResult, EasyJniError};
use jni::{
    objects::{JClass, JObject, JString, JValueOwned},
    JNIEnv,
};

/// The eight primitive types of java
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum JavaType {
    /// an 8-bit signed two's complement integer, ranging from -128 to 127
    Byte(u8),
    /// a 16-bit signed two's complement integer, ranging from -32,768 to 32,767
    Short(i16),
    /// a 32-bit signed two's complement integer, ranging from -2,147,483,648 to 2,147,483,647
    Int(i32),
    /// a 64-bit signed two's complement integer, ranging from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    Long(i64),
    /// a single-precision 32-bit IEEE 754 floating-point number
    Float(f32),
    /// a double-precision 64-bit IEEE 754 floating-point number
    Double(f64),
    /// a data type that can only take on the values true or false
    Boolean(bool),
    /// a single 16-bit Unicode character, representing a wide range of characters from different languages and scripts.
    Char(char),
    /// Returns nothing
    Void,
    /// A String
    String(String),
}

impl Default for JavaType {
    fn default() -> Self {
        JavaType::Void
    }
}

impl<'local> JavaType {
    pub fn new_string(
        env: &JNIEnv<'local>,
        _: &JClass<'local>,
        value: &str,
    ) -> EasyJNIResult<JString<'local>> {
        Ok(env.new_string(&value)?)
    }

    pub fn new_jvalue_string(
        env: &JNIEnv<'local>,
        _: &JClass<'local>,
        value: &str,
    ) -> EasyJNIResult<JValueOwned<'local>> {
        let string = env.new_string(&value)?;

        Ok(JValueOwned::from(JString::from(string)))
    }

    pub fn to_jni_jvalue(
        &self,
        env: &JNIEnv<'local>,
        _: &JClass<'local>,
    ) -> EasyJNIResult<JValueOwned<'local>> {
        let outcome = match self {
            Self::Byte(byte_value) => JValueOwned::from(*byte_value as i8),
            Self::Short(short_value) => JValueOwned::from(*short_value),
            Self::Int(int_value) => JValueOwned::from(*int_value),
            Self::Long(long_value) => JValueOwned::from(*long_value),
            Self::Float(float_value) => JValueOwned::from(*float_value),
            Self::Double(double_value) => JValueOwned::from(*double_value),
            Self::Boolean(bool_value) => JValueOwned::from(*bool_value),
            Self::Void => JValueOwned::Void,
            Self::String(string_value) => JValueOwned::from(env.new_string(string_value)?),
            _ => return Err(EasyJniError::UnsupportedJavaType),
        };

        Ok(outcome)
    }

    pub fn to_jni_object(
        &'local self,
        env: &mut JNIEnv<'local>,
        java_class: &'local JClass<'local>,
    ) -> EasyJNIResult<JObject<'local>> {
        let class = self.java_class(env, &java_class)?;

        let object = env.alloc_object(class)?;

        Ok(object)
    }

    pub fn java_class(
        &'local self,
        env: &mut JNIEnv<'local>,
        _: &JClass<'local>,
    ) -> EasyJNIResult<JClass> {
        // Get a reference to the Integer class
        Ok(env.find_class(self.java_class_name())?)
    }

    pub fn java_signature(&self) -> &str {
        match self {
            Self::Byte { .. } => JavaTypeSignature::Byte.java_signature(),
            Self::Short { .. } => JavaTypeSignature::Short.java_signature(),
            Self::Int { .. } => JavaTypeSignature::Int.java_signature(),
            Self::Long { .. } => JavaTypeSignature::Long.java_signature(),
            Self::Float { .. } => JavaTypeSignature::Float.java_signature(),
            Self::Double { .. } => JavaTypeSignature::Double.java_signature(),
            Self::Boolean { .. } => JavaTypeSignature::Boolean.java_signature(),
            Self::Char { .. } => JavaTypeSignature::Char.java_signature(),
            Self::Void { .. } => JavaTypeSignature::Void.java_signature(),
            Self::String { .. } => JavaTypeSignature::String.java_signature(),
        }
    }

    pub fn java_class_name(&self) -> String {
        self.to_java_type_signature().java_class_name().to_owned()
    }

    pub fn to_java_type_signature(&self) -> JavaTypeSignature {
        match self {
            JavaType::Byte(_) => JavaTypeSignature::Byte,
            JavaType::Short(_) => JavaTypeSignature::Short,
            JavaType::Int(_) => JavaTypeSignature::Int,
            JavaType::Long(_) => JavaTypeSignature::Long,
            JavaType::Float(_) => JavaTypeSignature::Float,
            JavaType::Double(_) => JavaTypeSignature::Double,
            JavaType::Boolean(_) => JavaTypeSignature::Boolean,
            JavaType::Char(_) => JavaTypeSignature::Char,
            JavaType::Void => JavaTypeSignature::Void,
            JavaType::String(_) => JavaTypeSignature::String,
        }
    }
}

#[derive(Debug, PartialEq, Default, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum JavaTypeSignature {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Char,
    #[default]
    Void,
    String,
}

impl JavaTypeSignature {
    pub fn java_signature(&self) -> &str {
        match self {
            Self::Byte => "B",
            Self::Short => "S",
            Self::Int => "I",
            Self::Long => "J",
            Self::Float => "F",
            Self::Double => "D",
            Self::Boolean => "Z",
            Self::Char => "C",
            Self::Void => "V",
            Self::String => "java/lang/String",
        }
    }

    pub fn java_class_name(&self) -> &str {
        match self {
            Self::Byte => "java/lang/Byte",
            Self::Short => "java/lang/Short",
            Self::Int => "java/lang/Integer",
            Self::Long => "java/lang/Long",
            Self::Float => "java/lang/Float",
            Self::Double => "java/lang/Double",
            Self::Boolean => "java/lang/Boolean",
            Self::Char => "java/lang/Character",
            Self::Void => "java/lang/Void",
            Self::String => "Ljava/lang/String;",
        }
    }

    pub fn java_class_name_array(&self) -> EasyJNIResult<&str> {
        let class_name = match self {
            Self::Byte => "[B",
            Self::Short => "[S",
            Self::Int => "[I",
            Self::Long => "[J",
            Self::Float => "[F",
            Self::Double => "[D",
            Self::Boolean => "[Z",
            Self::Char => "[C",
            Self::Void => return Err(EasyJniError::ArrayOfVoidNotAllowed),
            Self::String => "[Ljava/lang/String;",
        };

        Ok(class_name)
    }

    pub fn to_jni_object<'local>(
        &self,
        env: &mut JNIEnv<'local>,
        _: &'local JClass<'local>,
    ) -> EasyJNIResult<JObject<'local>> {
        let class = self.java_class_name();

        let object = env.alloc_object(class)?;

        Ok(object)
    }
}
