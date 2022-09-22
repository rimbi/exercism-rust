pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = self.0.replace(' ', "");
        if code.len() <= 1 {
            return false;
        }
        if code.as_bytes().iter().any(|&c| !(b'0'..b'9').contains(&c)) {
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
