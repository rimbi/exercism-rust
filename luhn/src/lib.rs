/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        return false;
    }
    if code.as_bytes().iter().any(|&c| c < b'0' || c > b'9') {
        return false;
    }

    let sum = code
        .as_bytes()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &c)| {
            let mut val = c - b'0';
            if i % 2 != 0 {
                val *= 2;
                if val > 9 {
                    val -= 9
                }
            }
            val
        })
        .sum::<u8>();
    sum % 10 == 0
}
