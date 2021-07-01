pub fn is_armstrong_number(num: u32) -> bool {
    let x = num.to_string();
    x.chars()
        .map(|c| c.to_digit(10).unwrap().pow(x.len() as u32))
        .sum::<u32>()
        == num
}
