const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(input: &[u8]) -> String {
    let mut result = Vec::new();
    let mut i = 0;
    while i < input.len() {
        let b1 = input[i];
        let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };
        let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
        result.push(BASE64_CHARS[((n >> 18) & 63) as usize]);
        result.push(BASE64_CHARS[((n >> 12) & 63) as usize]);
        if i + 1 < input.len() {
            result.push(BASE64_CHARS[((n >> 6) & 63) as usize]);
        } else {
            result.push(b'=');
        }
        if i + 2 < input.len() {
            result.push(BASE64_CHARS[(n & 63) as usize]);
        } else {
            result.push(b'=');
        }
        i += 3;
    }
    String::from_utf8(result).unwrap_or_default()
}

pub fn decode(input: &str) -> Result<Vec<u8>, &'static str> {
    let mut result = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let mut n = 0u32;
        let mut bits = 0;
        for j in 0..4 {
            if i + j >= bytes.len() {
                return Err("Invalid base64 length");
            }
            let c = bytes[i + j];
            let val = if c.is_ascii_uppercase() {
                c - b'A'
            } else if c.is_ascii_lowercase() {
                c - b'a' + 26
            } else if c.is_ascii_digit() {
                c - b'0' + 52
            } else if c == b'+' {
                62
            } else if c == b'/' {
                63
            } else if c == b'=' {
                0
            } else {
                return Err("Invalid base64 character");
            };
            n |= (val as u32) << (18 - j * 6);
            if c != b'=' {
                bits += 6;
            }
        }
        result.push((n >> 16) as u8);
        if bits > 8 {
            result.push((n >> 8) as u8);
        }
        if bits > 16 {
            result.push(n as u8);
        }
        i += 4;
    }
    Ok(result)
}
