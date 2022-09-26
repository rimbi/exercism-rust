use rand::Rng;

pub fn encode_decode<F>(key: &str, s: &str, op: F) -> Option<String>
where
    F: Fn(char, char) -> char,
{
    if key.is_empty()
        || s.chars().any(|c| !('a'..='z').contains(&c))
        || key.chars().any(|c| !('a'..='z').contains(&c))
    {
        return None;
    }
    let res = s
        .to_ascii_lowercase()
        .chars()
        .zip(key.to_ascii_lowercase().chars().cycle())
        .map(|(c, k)| if !c.is_alphabetic() { c } else { op(c, k) })
        .collect::<String>();
    Some(res)
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    encode_decode(key, s, |c, k| {
        let d = (((k as u32 - 'a' as u32) + c as u32) - 'a' as u32) % 26 + 'a' as u32;
        char::from_u32(d).unwrap()
    })
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    encode_decode(key, s, |c, k| {
        let d = ((c as u32 - (k as u32 - 'a' as u32)) + 26 - 'a' as u32) % 26 + 'a' as u32;
        char::from_u32(d).unwrap()
    })
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key: String = std::iter::from_fn(|| Some(rng.gen_range('a'..='z')))
        .take(100)
        .collect();
    let cipher = encode(&key, s).unwrap();
    (key, cipher)
}
