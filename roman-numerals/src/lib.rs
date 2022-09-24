use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut res = String::new();
        res += &map_digit(num / 1000, 'M', ' ', ' ');
        res += &map_digit((num % 1000) / 100, 'C', 'D', 'M');
        res += &map_digit((num % 100) / 10, 'X', 'L', 'C');
        res += &map_digit(num % 10, 'I', 'V', 'X');
        Self(res)
    }
}

fn map_digit(hs: u32, ch1: char, ch2: char, ch3: char) -> String {
    let mut res = String::new();
    match hs {
        n @ 1..=3 => {
            for _ in 0..n {
                res.push(ch1);
            }
        }
        4 => res.extend([ch1, ch2]),
        5 => res.push(ch2),
        n @ 6..=8 => {
            res.push(ch2);
            for _ in 0..(n - 5) {
                res.push(ch1);
            }
        }
        9 => res.extend([ch1, ch3]),
        _ => {}
    }
    res
}
