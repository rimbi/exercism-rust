#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    let max = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<_>, Error>>()?
        .windows(span)
        .map(|w| w.iter().map(|c| *c as u64).product())
        .max()
        .unwrap();
    Ok(max)
}
