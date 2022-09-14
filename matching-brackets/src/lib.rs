fn get_complementary(ch: char) -> Option<char> {
    match ch {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        _ => None
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for ch in string.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                if stack.pop() != get_complementary(ch) {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
