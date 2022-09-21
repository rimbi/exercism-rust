pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    // Eliminate unused characters
    let mut input = input
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<Vec<_>>();
    // Find the proper row/count values
    let (r, c) = (1..)
        .into_iter()
        .find_map(|i: usize| {
            if i.pow(2) >= input.len() {
                return Some((i, i));
            }
            if i * (i + 1) >= input.len() {
                return Some((i, i + 1));
            }
            None
        })
        .unwrap();
    // Pad the string if necessary 
    // in accordance with the size of the rectangle (r x c)
    let len = input.len();
    for _ in 0..(c * r - len) {
        input.push(' ');
    }

    // Exchange rows and columns
    let mut res = vec![];
    for i in 0..c {
        for s in input.chunks(c) {
            res.push(s[i])
        }
    }
    // Form the resulting string by joining the rows with space
    res.chunks(r)
        .map(String::from_iter)
        .collect::<Vec<_>>()
        .join(" ")
}
