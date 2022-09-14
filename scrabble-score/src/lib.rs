use std::collections::HashMap;

static POINTS: &[(char, u64); 26] = &[
    ('A', 1),
    ('E', 1),
    ('I', 1),
    ('O', 1),
    ('U', 1),
    ('L', 1),
    ('N', 1),
    ('R', 1),
    ('S', 1),
    ('T', 1),
    ('D', 2),
    ('G', 2),
    ('B', 3),
    ('C', 3),
    ('M', 3),
    ('P', 3),
    ('F', 4),
    ('H', 4),
    ('V', 4),
    ('W', 4),
    ('Y', 4),
    ('K', 5),
    ('J', 8),
    ('X', 8),
    ('Q', 10),
    ('Z', 10),
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let map: HashMap<char, u64> = POINTS.iter().cloned().collect();
    word.chars()
        .filter(char::is_ascii)
        .map(|c| map.get(&c.to_ascii_uppercase()).map(|&p| p).unwrap_or(0))
        .sum::<u64>()
}
