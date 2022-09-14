pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = num.to_string().len();
    let mut remaining = num;
    let mut total = 0;
    while remaining > 0 {
        total += (remaining % 10).pow(num_digits as u32);
        remaining /= 10;
    }
    total == num
}
