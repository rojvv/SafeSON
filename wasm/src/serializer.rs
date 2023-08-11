use js_sys::Array;
use js_sys::JsString;
use js_sys::Object;

use crate::rle;
use crate::Value;
use crate::Value::*;
use crate::ARRAY;
use crate::FALSE;
use crate::NULL;
use crate::NUMBER;
use crate::OBJECT;
use crate::STRING;
use crate::TRUE;

pub(crate) struct Serializer {
    buffer: Vec<u8>,
}

impl Serializer {
    fn new() -> Serializer {
        Serializer {
            buffer: Vec::with_capacity(24),
        }
    }

    fn write(&mut self, b: &[u8]) {
        self.buffer.extend(b);
    }

    fn write_boolean(&mut self, boolean: &bool) {
        if *boolean {
            self.write(&[TRUE]);
        } else {
            self.write(&[FALSE]);
        }
    }

    fn write_null(&mut self) {
        self.write(&[NULL])
    }

    fn write_number_inner(&mut self, number: &f64) {
        self.write(&number.to_le_bytes())
    }

    fn write_number(&mut self, number: &f64) {
        self.write(&[NUMBER]);
        self.write_number_inner(number)
    }

    fn write_length(&mut self, length: &f64) {
        if *length <= 254.0 {
            self.write(&[*length as u8])
        } else {
            self.write(&[255]);
            self.write_number_inner(length);
        }
    }

    fn write_string_inner(&mut self, string: &JsString) {
        self.write_length(&(string.length() as f64));
        self.write(string.as_string().unwrap().as_bytes());
    }

    fn write_string(&mut self, string: &JsString) {
        self.write(&[STRING]);
        self.write_string_inner(string);
    }

    fn write_array(&mut self, array: &Array) -> Result<(), ()> {
        self.write(&[ARRAY]);
        self.write_length(&(array.length() as f64));
        for v in array.iter() {
            self.write_value(&Value::try_from(v)?)?;
        }
        Ok(())
    }

    fn write_object(&mut self, object: &Object) -> Result<(), ()> {
        self.write(&[OBJECT]);
        self.write_length(&(Object::keys(object).length() as f64));
        for v in Object::entries(object) {
            let kv = Array::try_from(v).unwrap();
            let k = JsString::from(kv.get(0));
            self.write_string_inner(&k);
            let v = Value::try_from(kv.get(1))?;
            self.write_value(&v)?;
        }
        Ok(())
    }

    fn write_value(&mut self, value: &Value) -> Result<(), ()> {
        match value {
            Boolean(v) => {
                self.write_boolean(v);
                Ok(())
            }
            Null => {
                self.write_null();
                Ok(())
            }
            Number(v) => {
                self.write_number(v);
                Ok(())
            }
            String(v) => {
                self.write_string(v);
                Ok(())
            }
            Array(v) => self.write_array(v),
            Object(v) => self.write_object(v),
        }
    }

    pub(crate) fn serialize(value: &Value) -> Result<Vec<u8>, ()> {
        let mut serializer = Serializer::new();
        serializer.write_value(value)?;
        Ok(rle::encode(&serializer.buffer))
    }
}

#[cfg(test)]
mod tests {
    use js_sys::JsString;
    use js_sys_util::create_array;
    use js_sys_util::create_boolean;
    use js_sys_util::create_entry;
    use js_sys_util::create_null;
    use js_sys_util::create_number;
    use js_sys_util::create_object;
    use js_sys_util::create_string;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::ARRAY;
    use crate::FALSE;
    use crate::NULL;
    use crate::NUMBER;
    use crate::OBJECT;
    use crate::STRING;
    use crate::TRUE;

    use super::Serializer;

    #[wasm_bindgen_test]
    fn test_write() {
        let mut serializer = Serializer::new();
        serializer.write(&[0, 1, 2, 3]);
        assert_eq!(serializer.buffer, &[0, 1, 2, 3]);
    }

    #[wasm_bindgen_test]
    fn test_write_boolean() {
        let mut serializer = Serializer::new();
        serializer.write_boolean(&false);
        serializer.write_boolean(&true);
        assert_eq!(serializer.buffer, &[FALSE, TRUE]);
    }

    #[wasm_bindgen_test]
    fn test_write_null() {
        let mut serializer = Serializer::new();
        serializer.write_null();
        assert_eq!(serializer.buffer, &[NULL])
    }

    #[wasm_bindgen_test]
    fn test_write_number_inner() {
        let mut serializer = Serializer::new();
        serializer.write_number_inner(&123.456);
        assert_eq!(
            serializer.buffer,
            &[0x77, 0xBE, 0x9F, 0x1A, 0x2F, 0xDD, 0x5E, 0x40]
        );
    }

    #[wasm_bindgen_test]
    fn test_write_number() {
        let mut serializer = Serializer::new();
        serializer.write_number(&123.456);
        assert_eq!(
            serializer.buffer,
            &[NUMBER, 0x77, 0xBE, 0x9F, 0x1A, 0x2F, 0xDD, 0x5E, 0x40]
        );
    }

    #[wasm_bindgen_test]
    fn test_write_length() {
        let mut serializer = Serializer::new();
        serializer.write_length(&254.0);
        serializer.write_length(&255.0);
        assert_eq!(
            serializer.buffer,
            &[254, 255, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x6F, 0x40]
        );
    }

    #[wasm_bindgen_test]
    fn test_write_string_inner() {
        let mut serializer = Serializer::new();
        serializer.write_string_inner(&JsString::from("Hello, world!".to_string()));
        serializer.write_string_inner(&JsString::from("Hello, world!".repeat(20)));

        let mut expected = Vec::new();
        expected.push(13);
        expected.extend("Hello, world!".as_bytes());
        expected.extend(&[255, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x70, 0x40]);
        expected.extend("Hello, world!".repeat(20).as_bytes());
        assert_eq!(serializer.buffer, expected);
    }

    #[wasm_bindgen_test]
    fn test_write_string() {
        let mut serializer = Serializer::new();
        serializer.write_string(&JsString::from("Hello, world!".to_string()));

        let mut expected = Vec::new();
        expected.push(STRING);
        expected.push(13);
        expected.extend("Hello, world!".as_bytes());

        assert_eq!(serializer.buffer, expected);

        let mut serializer = Serializer::new();
        serializer.write_string(&JsString::from("Hello, world!".repeat(20)));

        let mut expected = Vec::new();
        expected.extend(&[STRING, 255, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x70, 0x40]);
        expected.extend("Hello, world!".repeat(20).as_bytes());
        assert_eq!(serializer.buffer, expected);
    }

    #[wasm_bindgen_test]
    fn test_write_array() {
        let array = create_array(&[
            &create_boolean(false),
            &create_boolean(true),
            &create_null(),
            &create_number(1_000.0),
            &create_string("Hello, world!"),
            &create_object(&[
                &create_entry("key", &create_boolean(false)),
                &create_entry("key2", &create_boolean(true)),
                &create_entry("key3", &create_null()),
                &create_entry("key4", &create_number(1_000.0)),
                &create_entry("key5", &create_string("Hello, world!")),
            ]),
        ]);
        let mut serializer = Serializer::new();
        serializer.write_array(&array).unwrap();
        let expected = &[
            ARRAY, //
            6u8,   // length
            FALSE, //
            TRUE,  //
            NULL,  //
            NUMBER, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x8F, 0x40, // number, 1_000
            STRING, 13, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64,
            0x21,   // string, Hello, world!
            OBJECT, //
            5,      // length
            3, 0x6B, 0x65, 0x79,  // key
            FALSE, //
            4, 0x6B, 0x65, 0x79, 0x32, // key2
            TRUE, //
            4, 0x6B, 0x65, 0x79, 0x33, // key3
            NULL, //
            4, 0x6B, 0x65, 0x79, 0x34, // key4
            NUMBER, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x8F, 0x40, // number, 1_000
            4, 0x6B, 0x65, 0x79, 0x35, // key5
            STRING, 13, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64,
            0x21, // string, Hello, world!
        ];
        assert_eq!(serializer.buffer, expected);
    }
}
