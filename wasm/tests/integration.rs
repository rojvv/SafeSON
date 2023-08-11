use js_sys::JSON;
use js_sys_util::create_array;
use js_sys_util::create_boolean;
use js_sys_util::create_entry;
use js_sys_util::create_number;
use js_sys_util::create_object;
use js_sys_util::create_string;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

use rbuf::deserialize;
use rbuf::serialize;
use rbuf::Value;

fn v(v: Value) {
    let v: JsValue = v.into();
    let serialized = serialize(v.clone());
    let expected = JSON::stringify(&v).unwrap().as_string().unwrap();
    let actual = JSON::stringify(&deserialize(&serialized))
        .unwrap()
        .as_string()
        .unwrap();
    assert_eq!(actual, expected);
}

#[wasm_bindgen_test]
fn ser_deser_boolean() {
    v(Value::Boolean(false));
    v(Value::Boolean(true));
}

#[wasm_bindgen_test]
fn ser_deser_null() {
    v(Value::Null);
}

#[wasm_bindgen_test]
fn ser_deser_number() {
    v(Value::Number(1.0));
    v(Value::Number(-273.0));
    v(Value::Number(123123.9123321));
}

#[wasm_bindgen_test]
fn ser_deser_string() {
    v(Value::String(create_string("Hello, world!")));
    v(Value::String(create_string(&"Hello, world!".repeat(30))));
}

#[wasm_bindgen_test]
fn ser_deser_array() {
    v(Value::Array(create_array(&[])));
    v(Value::Array(create_array(&[
        &create_string("Hello, world!"),
        &create_number(-12312342.213),
        &create_boolean(true),
        &create_boolean(false),
        &create_object(&[&create_entry(
            "k",
            &create_array(&[&create_number(12345.6789)]),
        )]),
    ])));
}

#[wasm_bindgen_test]
fn ser_deser_object() {
    v(Value::Object(create_object(&[&create_entry(
        "key",
        &create_string("Hello, world!"),
    )])));
}
