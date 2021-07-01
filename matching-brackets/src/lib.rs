use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut v = vec![];
    let brackets: HashMap<char, char> = [('{', '}'), ('[', ']'), ('(', ')')]
        .iter()
        .cloned()
        .collect();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => {
                v.push(c);
            }
            '}' | ']' | ')' => {
                if Some(x) != v.pop() {
                    return false;
                }
            }
            _ => (),
        }
    }
    v.is_empty()
}
