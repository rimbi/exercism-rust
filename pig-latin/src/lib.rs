use std::{collections::HashSet, iter::FromIterator};
use lazy_static::lazy_static;

lazy_static! {
    static ref VOWELS: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    static ref ALL_LETTERS: HashSet<char> = HashSet::<char>::from_iter('a'..='z');
    static ref CONSONANTS: HashSet<char> = ALL_LETTERS.difference(&VOWELS).cloned().collect();
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(convert_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn convert_word(input: &str) -> String {
    // Rule 1
    if VOWELS.contains(&input.chars().next().unwrap())
        || input.starts_with("xr")
        || input.starts_with("yt")
    {
        return format!("{}ay", input);
    }
    // Rule 2
    let consonants_at_start = input
        .chars()
        .take_while(|c| CONSONANTS.contains(c))
        .collect::<String>();
    if !consonants_at_start.is_empty() {
        let rest = &input[consonants_at_start.len()..];
        // Rule 4
        if let Some(pos) = consonants_at_start.find('y') {
            if pos > 0 {
                return format!(
                    "{}{}{}ay",
                    &consonants_at_start[pos..],
                    rest,
                    &consonants_at_start[..pos]
                );
            }
        }
        // Rule 3
        if rest.starts_with('u') && consonants_at_start.ends_with('q') {
            return format!("{}{}uay", &rest[1..], consonants_at_start);
        }
        return format!("{}{}ay", rest, consonants_at_start);
    }
    input.to_string()
}
