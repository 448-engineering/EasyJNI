use crate::{EasyJNIResult, EasyJniError};
use jni::{
    objects::{JClass, JObject, JValueOwned},
    strings::JNIString,
    sys::{jarray, jint, jsize},
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
        &self,
        env: &JNIEnv<'local>,
        java_class: &JClass<'local>,
    ) -> EasyJNIResult<JObject<'local>> {
        let class = self.java_class(env, java_class)?;
        let signature = self.java_signature();
        let object_value = self.to_jni_jvalue(env, java_class)?;

        let object = env.new_object(class, signature, &[object_value.borrow()])?;

        Ok(object)
    }

    pub fn java_signature(&self) -> &str {
        match self {
            Self::Byte { .. } => "B",
            Self::Short { .. } => "S",
            Self::Int { .. } => "I",
            Self::Long { .. } => "J",
            Self::Float { .. } => "F",
            Self::Double { .. } => "D",
            Self::Boolean { .. } => "Z",
            Self::Char { .. } => "C",
            Self::Void { .. } => "V",
            Self::String { .. } => "java/lang/String",
        }
    }

    pub fn java_class_name(&self) -> &str {
        match self {
            Self::Byte { .. } => "java/lang/Byte",
            Self::Short { .. } => "java/lang/Short",
            Self::Int { .. } => "java/lang/Integer",
            Self::Long { .. } => "java/lang/Long",
            Self::Float { .. } => "java/lang/Float",
            Self::Double { .. } => "java/lang/Double",
            Self::Boolean { .. } => "java/lang/Boolean",
            Self::Char { .. } => "java/lang/Character",
            Self::Void { .. } => "java/lang/Void",
            Self::String { .. } => "java/lang/String;",
        }
    }

    pub fn java_class(
        &'local self,
        env: &JNIEnv<'local>,
        _: &JClass<'local>,
    ) -> EasyJNIResult<JClass> {
        // Get a reference to the Integer class
        Ok(env.find_class(self.java_class_name())?)
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct JavaArray {
    size: usize,
    java_type: JavaType,
    values: Vec<JavaType>,
}

impl<'local> JavaArray {
    pub fn new(java_type: JavaType) -> Self {
        JavaArray {
            size: 0,
            java_type,
            values: Vec::default(),
        }
    }

    pub fn add_value(mut self, value: JavaType) -> Self {
        self.values.push(value);

        self
    }

    pub fn create(self, env: &JNIEnv<'local>, java_class: JClass<'local>) -> EasyJNIResult<jarray> {
        let class = self.java_type.java_class(&env, &java_class)?;

        let jarray = env.new_object_array(self.size as jsize, class, JObject::null())?;

        for (i, s) in self.values.iter().enumerate() {
            let object = s.to_jni_object(&env, &java_class)?;

            env.set_object_array_element(&jarray, i as jint, object)?;
        }

        Ok(jarray.into_raw())
    }

    pub fn java_signature(&self) -> EasyJNIResult<&str> {
        let signature = match self.java_type {
            JavaType::Byte { .. } => "[B",
            JavaType::Short { .. } => "[S",
            JavaType::Int { .. } => "[I",
            JavaType::Long { .. } => "[J",
            JavaType::Float { .. } => "[F",
            JavaType::Double { .. } => "[D",
            JavaType::Boolean { .. } => "[Z",
            JavaType::Char { .. } => "[C",
            JavaType::Void => return Err(EasyJniError::ArrayOfVoidNotAllowed),
            JavaType::String { .. } => "[java/lang/String",
        };

        Ok(signature)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Class<'local> {
    name: &'local str,
    fields: Vec<(&'local str, JavaType)>,
}

impl<'local> Default for Class<'local> {
    fn default() -> Self {
        Class {
            name: "UNINITIALIZED_CLASS_NAME",
            fields: vec![("UNINITALIZED_CLASS_FIELD", JavaType::Void)],
        }
    }
}

impl<'local> Class<'local> {
    pub fn new() -> Self {
        Class::default()
    }
    pub fn create(
        self,
        env: &mut JNIEnv<'local>,
        java_class: JClass<'local>,
    ) -> EasyJNIResult<JObject<'local>> {
        let class = env.find_class(&self.name)?;

        let object = env.alloc_object(class)?;

        for (name, value) in &self.fields {
            let field_name = JNIString::from(name);

            env.set_field(
                &object,
                field_name,
                value.java_signature(),
                value.to_jni_jvalue(&env, &java_class)?.borrow(),
            )?;
        }

        Ok(object)
    }
}
