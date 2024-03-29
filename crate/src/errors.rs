use jni::errors::Error as JniError;

pub type EasyJNIResult<T> = Result<T, EasyJniError>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EasyJniError {
    ArrayOfVoidNotAllowed,
    UnsupportedJavaType,
    /// Any Java byte (i8) value less than 0 will give a hard error since Rust byte is unsigned.
    ByteLessThanZeroNotSupported,
    /// The `JavaType` does not match the required type
    JavaTypeMismatch,
    FromJniCrate(String),
    /// Tried to create a class without adding the class name first
    ClassNameRequiredToCreateClass,
}

impl From<JniError> for EasyJniError {
    fn from(value: JniError) -> Self {
        EasyJniError::FromJniCrate(value.to_string())
    }
}
