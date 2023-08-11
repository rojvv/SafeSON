mod deserializer;
mod rle;
mod serializer;

use deserializer::Deserializer;
use js_sys::Array;
use js_sys::JsString;
use js_sys::Object;
use wasm_bindgen::prelude::*;

use serializer::Serializer;
use wasm_bindgen::throw_str;

type Result<T> = std::result::Result<T, &'static str>;

pub(crate) const FALSE: u8 = 0;
pub(crate) const TRUE: u8 = 1;
pub(crate) const NULL: u8 = 2;
pub(crate) const NUMBER: u8 = 3;
pub(crate) const STRING: u8 = 4;
pub(crate) const ARRAY: u8 = 5;
pub(crate) const OBJECT: u8 = 6;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Null,
    Number(f64),
    String(JsString),
    Array(Array),
    Object(Object),
}

impl TryFrom<JsValue> for Value {
    type Error = ();

    fn try_from(value: JsValue) -> std::result::Result<Self, Self::Error> {
        if let Some(v) = value.as_bool() {
            Ok(Value::Boolean(v))
        } else if let Some(v) = value.dyn_ref::<JsString>() {
            Ok(Value::String(v.clone()))
        } else if value.is_null() {
            Ok(Value::Null)
        } else if let Some(v) = value.as_f64() {
            Ok(Value::Number(v))
        } else if let Some(v) = value.dyn_ref::<Array>() {
            Ok(Value::Array(v.clone()))
        } else if let Some(v) = value.dyn_ref::<Object>() {
            Ok(Value::Object(v.clone()))
        } else {
            Err(())
        }
    }
}

impl From<Value> for JsValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Boolean(v) => {
                if v {
                    JsValue::TRUE
                } else {
                    JsValue::FALSE
                }
            }
            Value::Null => JsValue::NULL,
            Value::Number(v) => js_sys::Number::from(v).into(),
            Value::String(v) => v.into(),
            Value::Array(v) => v.into(),
            Value::Object(v) => v.into(),
        }
    }
}

#[wasm_bindgen]
pub fn serialize(v: JsValue) -> Vec<u8> {
    if let Ok(v) = Value::try_from(v) {
        if let Ok(v) = Serializer::serialize(&v) {
            return v;
        }
    }

    Vec::new()
}

#[wasm_bindgen]
pub fn deserialize(v: &[u8]) -> JsValue {
    match Deserializer::deserialize(v) {
        Ok(v) => v,
        Err(e) => throw_str(e),
    }
}
