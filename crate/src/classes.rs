use crate::{EasyJNIResult, EasyJniError, JavaType, JavaTypeSignature};
use jni::{
    objects::{JClass, JObject, JObjectArray, JValueOwned},
    strings::JNIString,
    sys::{jarray, jint, jsize},
    JNIEnv,
};

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct JavaArray {
    size: usize,
    java_type: JavaTypeSignature,
    values: Vec<JavaType>,
}

impl<'local> JavaArray {
    pub fn new(java_type: JavaTypeSignature) -> Self {
        JavaArray {
            size: 0,
            java_type,
            values: Vec::default(),
        }
    }

    pub fn resize(mut self, size: usize) -> Self {
        self.size = size;

        self
    }

    pub fn add_value(mut self, value: JavaType) -> EasyJNIResult<Self> {
        if value.to_java_type_signature() != self.java_type {
            return Err(EasyJniError::JavaTypeMismatch);
        }

        self.values.push(value);

        Ok(self)
    }

    pub fn values(&self) -> &Vec<JavaType> {
        self.values.as_ref()
    }

    pub fn create(
        env: &mut JNIEnv<'local>,
        _: &JClass<'local>,
        java_type: JavaTypeSignature,
        size: usize,
    ) -> EasyJNIResult<JObjectArray<'local>> {
        let java_signature = java_type.java_class_name();

        let jarray = env.new_object_array(size as jsize, java_signature, JObject::null())?;

        Ok(jarray)
    }

    pub fn build(
        &'local self,
        env: &mut JNIEnv<'local>,
        java_class: &'local JClass<'local>,
    ) -> EasyJNIResult<jarray> {
        let class = self.java_type.java_class_name();

        let jarray = env.new_object_array(self.size as jsize, class, JObject::null())?;

        for (i, s) in self.values.iter().enumerate() {
            let object = s.to_jni_object(env, &java_class)?;

            env.set_object_array_element(&jarray, i as jint, object)?;
        }

        Ok(jarray.into_raw())
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Class<'local> {
    name: &'local str,
}

impl<'local> Class<'local> {
    pub fn new(name: &'local str) -> Class {
        Class { name }
    }

    pub fn name(&self) -> &'local str {
        self.name
    }

    pub fn create(
        &self,
        env: &mut JNIEnv<'local>,
        _java_class: &JClass<'local>,
    ) -> EasyJNIResult<JObject<'local>> {
        let class = env.find_class(&self.name)?;

        let object = env.alloc_object(class)?;

        Ok(object)
    }

    pub fn create_and_build(
        &'local self,
        env: &mut JNIEnv<'local>,
        java_class: &'local JClass<'local>,
        name: &str,
        value: &'local JavaType,
    ) -> EasyJNIResult<JObject<'local>> {
        let class = env.find_class(&self.name)?;

        let object = env.alloc_object(class)?;

        let field_name = JNIString::from(name);
        let field_value = value.to_jni_object(env, &java_class)?;
        let field_value = JValueOwned::Object(field_value);

        env.set_field(
            &object,
            field_name,
            value.java_class_name(),
            field_value.borrow(),
        )?;

        Ok(object)
    }

    pub fn find(
        self,
        env: &mut JNIEnv<'local>,
        _: JClass<'local>,
    ) -> EasyJNIResult<JClass<'local>> {
        Ok(env.find_class(&self.name)?)
    }
}
