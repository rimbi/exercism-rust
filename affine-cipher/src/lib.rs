/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let m = 'z' as i32 - 'a' as i32 + 1;
    find_mmi(a, m)?;
    let cipher = plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(|c| {
            let c = if ('0'..='9').contains(&c) {
                c as i32
            } else {
                ((a * (c as i32 - 'a' as i32) + b) % m) + 'a' as i32
            } as u32;
            char::from_u32(c)
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(String::from_iter)
        .collect::<Vec<String>>()
        .join(" ");

    Ok(cipher)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let m = 'z' as i32 - 'a' as i32 + 1;
    let mmi = find_mmi(a, m)?;
    let plaintext = ciphertext
        .replace(' ', "")
        .chars()
        .map(|c| {
            if ('0'..='9').contains(&c) {
                c as u32
            } else {
                let c = c as i32 - 'a' as i32;
                let mut decoded = (mmi * (c - b)) % m;
                if decoded < 0 {
                    decoded += m;
                }
                decoded as u32 + 'a' as u32
            }
        })
        .flat_map(char::from_u32)
        .collect::<String>();
    Ok(plaintext)
}

fn find_mmi(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    (1..m)
        .find(|i| (a * i) % m == 1)
        .ok_or(AffineCipherError::NotCoprime(a))
}

// fn check_coprime(a: i32, m: i32) -> Result<(), AffineCipherError> {
//     let min = min(a, m);
//     let max = max(a, m);
//     if (2..=min).any(|i| min % i == 0 && max % i == 0) {
//         return Err(AffineCipherError::NotCoprime(a));
//     }
//     Ok(())
// }
