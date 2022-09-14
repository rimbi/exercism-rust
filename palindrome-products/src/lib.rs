use std::fmt::Debug;
/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let val_str = value.to_string();
        if val_str == val_str.chars().rev().collect::<String>() {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes = find_palindromes(min, max);
    palindromes.sort();
    Some((palindromes.first().cloned()?, palindromes.last().cloned()?))
}

fn find_palindromes(min: u64, max: u64) -> Vec<Palindrome> {
    (min..=max)
        .flat_map(|i| (i..=max).filter_map(move |j| Palindrome::new(i * j)))
        .collect()
}
