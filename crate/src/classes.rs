use crate::{EasyJNIResult, EasyJniError};
use std::collections::HashMap;

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
    Void,
    String(String),
}

impl JavaType {
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
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct JavaArray {
    size: usize,
    primitive: JavaType,
}

impl JavaArray {
    pub fn java_signature(&self) -> EasyJNIResult<&str> {
        let signature = match self.primitive {
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
pub enum Class {
    Primitive {
        name: String,
        fields: Vec<(String, JavaType)>,
    },
    DataType {
        name: String,
        fields: Vec<(String, JavaType)>,
    },
}

#[derive(Debug, Clone)]
pub struct JavaCollection(HashMap<JavaType, JavaType>);
