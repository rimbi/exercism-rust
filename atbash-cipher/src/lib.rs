/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| {
            if c.is_ascii_digit() {
                Some(c)
            } else {
                let c = 'z' as u32 - (c as u32 - 'a' as u32);
                char::from_u32(c)
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(String::from_iter)
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .replace(' ', "")
        .chars()
        .flat_map(|c| {
            if c.is_ascii_digit() {
                Some(c)
            } else {
                let c = 'a' as u32 + ('z' as u32 - c as u32);
                char::from_u32(c)
            }
        })
        .collect::<String>()        
}
