use std::cmp::{min, max};

pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|i| i.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    let sum_of_squares = sum_of_squares(n);
    let square_of_sum = square_of_sum(n);
    max(sum_of_squares, square_of_sum) - min(sum_of_squares, square_of_sum)
}
