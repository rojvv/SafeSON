pub(crate) fn encode(v: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    let mut n = 0;
    for b in v {
        let b = *b;
        if b == 0 {
            if n == 0xFF {
                r.push(0);
                r.push(n);
                n = 1;
            } else {
                n += 1;
            }
        } else {
            if n != 0 {
                r.push(0);
                r.push(n);
                n = 0;
            }

            r.push(b)
        }
    }
    if n != 0 {
        r.push(0);
        r.push(n);
    }
    r
}

pub(crate) fn decode(v: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    let mut z = false;
    for b in v {
        let b = *b;
        if b == 0 {
            z = true;
            continue;
        }

        if z {
            for _ in 0..b {
                r.push(0)
            }
            z = false
        } else {
            r.push(b)
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_encode() {
        let actual = super::encode(&[0, 0, 0, 0, 1, 2, 3, 4]);
        let expected = &[0, 4, 1, 2, 3, 4];
        assert_eq!(actual, expected);

        let mut v = Vec::new();
        for _ in 0..260 {
            v.push(0);
        }
        v.extend(&[1, 0, 2, 3, 4]);
        for _ in 0..300 {
            v.push(0);
        }
        let actual = super::encode(&v);
        let expected = [0, 255, 0, 5, 1, 0, 1, 2, 3, 4, 0, 255, 0, 45];
        assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    pub fn decode() {
        let actual = super::decode(&[0, 4, 1, 2, 3, 4]);
        let expected = &[0, 0, 0, 0, 1, 2, 3, 4];
        assert_eq!(actual, expected);

        let mut expected = Vec::new();
        for _ in 0..260 {
            expected.push(0);
        }
        expected.extend(&[1, 0, 2, 3, 4]);
        for _ in 0..300 {
            expected.push(0);
        }
        let actual = super::decode(&[0, 255, 0, 5, 1, 0, 1, 2, 3, 4, 0, 255, 0, 45]);
        assert_eq!(actual, expected);
    }
}
