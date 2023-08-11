use js_sys::Array;
use js_sys::JsString;
use js_sys::Number;
use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::rle;
use crate::ARRAY;
use crate::FALSE;
use crate::NULL;
use crate::NUMBER;
use crate::OBJECT;
use crate::STRING;
use crate::TRUE;

pub(crate) struct Deserializer {
    buffer: Vec<u8>,
    offset: usize,
}

impl Deserializer {
    fn new(buffer: &[u8]) -> Deserializer {
        Deserializer {
            buffer: buffer.into(),
            offset: 0,
        }
    }

    fn read(&mut self, count: usize) -> &[u8] {
        let b = &self.buffer[self.offset..self.offset + count];
        self.offset += count;
        b
    }

    fn read_number_inner(&mut self) -> f64 {
        let b = self.read(8);
        let b = [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]];
        f64::from_le_bytes(b)
    }

    fn read_number(&mut self) -> Number {
        Number::from(self.read_number_inner())
    }

    fn read_length(&mut self) -> usize {
        let b = self.read(1)[0];
        if b <= 254 {
            b as usize
        } else {
            self.read_number_inner() as usize
        }
    }

    fn read_string(&mut self) -> JsString {
        let length = self.read_length();
        JsString::from(std::string::String::from_utf8_lossy(self.read(length)).to_string())
    }

    fn read_array(&mut self) -> Result<Array, ()> {
        let length = self.read_length();
        let array = Array::new();
        for _ in 0..length {
            array.push(&self.read_value()?);
        }
        Ok(array)
    }

    fn read_object(&mut self) -> Result<Object, ()> {
        let length = self.read_length();
        let entries = Array::new();
        for _ in 0..length {
            let key = self.read_string();
            let value = self.read_value()?;

            let entry = Array::new();
            entry.push(&key);
            entry.push(&value);

            entries.push(&entry);
        }
        Object::from_entries(&entries).map_err(|_| ())
    }

    fn read_value(&mut self) -> Result<JsValue, ()> {
        let r#type = self.read(1)[0];
        match r#type {
            TRUE => Ok(JsValue::TRUE),
            FALSE => Ok(JsValue::FALSE),
            NULL => Ok(JsValue::NULL),
            NUMBER => Ok(self.read_number().into()),
            STRING => Ok(self.read_string().into()),
            ARRAY => Ok(self.read_array()?.into()),
            OBJECT => Ok(self.read_object()?.into()),
            _ => Err(()),
        }
    }

    pub fn deserialize(buffer: &[u8]) -> Result<JsValue, ()> {
        Deserializer::new(&rle::decode(buffer)).read_value()
    }
}
