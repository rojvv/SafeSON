use js_sys::Array;
use js_sys::JsString;
use js_sys::Number;
use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::rle;
use crate::Result;
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

    fn read(&mut self, count: usize) -> Result<&[u8]> {
        if self.buffer.len() < self.offset + count {
            Err("No more data remaining")
        } else {
            let b = &self.buffer[self.offset..self.offset + count];
            self.offset += count;
            Ok(b)
        }
    }

    fn read_number_inner(&mut self) -> Result<f64> {
        let b = self.read(8)?;
        let b = [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]];
        Ok(f64::from_le_bytes(b))
    }

    fn read_number(&mut self) -> Result<Number> {
        Ok(Number::from(self.read_number_inner()?))
    }

    fn read_length(&mut self) -> Result<usize> {
        let b = self.read(1)?[0];
        if b <= 254 {
            Ok(b as usize)
        } else {
            let f = self.read_number_inner()?;
            Ok(f as usize)
        }
    }

    fn read_string(&mut self) -> Result<JsString> {
        let length = self.read_length()?;
        Ok(JsString::from(
            std::string::String::from_utf8_lossy(self.read(length)?).to_string(),
        ))
    }

    fn read_array(&mut self) -> Result<Array> {
        let length = self.read_length()?;
        let array = Array::new();
        for _ in 0..length {
            array.push(&self.read_value()?);
        }
        Ok(array)
    }

    fn read_object(&mut self) -> Result<Object> {
        let length = self.read_length()?;
        let entries = Array::new();
        for _ in 0..length {
            let key = self.read_string()?;
            let value = self.read_value()?;

            let entry = Array::new();
            entry.push(&key);
            entry.push(&value);

            entries.push(&entry);
        }
        Object::from_entries(&entries).map_err(|_| "Unexpected error")
    }

    fn read_value(&mut self) -> Result<JsValue> {
        let r#type = self.read(1)?[0];
        match r#type {
            FALSE => Ok(JsValue::FALSE),
            TRUE => Ok(JsValue::TRUE),
            NULL => Ok(JsValue::NULL),
            NUMBER => Ok(self.read_number()?.into()),
            STRING => Ok(self.read_string()?.into()),
            ARRAY => Ok(self.read_array()?.into()),
            OBJECT => Ok(self.read_object()?.into()),
            _ => Err("Invalid type"),
        }
    }

    fn check_buffer(buffer: &[u8]) -> Result<()> {
        if buffer.len() <= 0
            || ((buffer[0] == TRUE || buffer[0] == NULL) && buffer.len() != 1)
            || (buffer[0] == FALSE && (buffer.len() != 2 || buffer[1] != 1))
        {
            Err("Invalid length")
        } else if buffer[0] != NULL
            && buffer[0] != TRUE
            && buffer[0] != FALSE
            && buffer[0] != NUMBER
            && buffer[0] != STRING
            && buffer[0] != ARRAY
            && buffer[0] != OBJECT
        {
            Err("Invalid type")
        } else {
            Ok(())
        }
    }

    pub fn deserialize(buffer: &[u8]) -> Result<JsValue> {
        Deserializer::check_buffer(buffer)?;
        Deserializer::new(&rle::decode(buffer)).read_value()
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::FALSE;

    use super::Deserializer;

    #[wasm_bindgen_test]
    fn test_check_buffer() {
        assert_eq!(Deserializer::check_buffer(&[]), Err("Invalid length"));
        assert_eq!(Deserializer::check_buffer(&[FALSE]), Err("Invalid length"));
        assert_eq!(Deserializer::check_buffer(&[FALSE, 1]), Ok(()));

        assert_eq!(
            Deserializer::check_buffer(&[FALSE, 1, 25]),
            Err("Invalid length")
        );

        assert_eq!(
            Deserializer::check_buffer(&[FALSE, 1, 25]),
            Err("Invalid length")
        );

        for i in 7..=255 {
            assert_eq!(Deserializer::check_buffer(&[i]), Err("Invalid type"))
        }
    }

    #[wasm_bindgen_test]
    fn test_read() {
        let mut deserializer = Deserializer::new(&[1]);
        assert_eq!(deserializer.read(2), Err("No more data remaining"));
    }
}
