/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "")
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if i == 9 && c == 'X' {
                Some(10)
            } else {
                c.to_digit(10)
            }
        })
        .collect::<Vec<_>>();
    if isbn.len() != 10 {
        return false;
    }
    let sum = isbn
        .iter()
        .zip((1..11).rev())
        .map(|(x, y)| x * y)
        .sum::<u32>();
    sum % 11 == 0
}
