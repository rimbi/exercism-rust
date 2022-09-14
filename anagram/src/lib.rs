use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    let lc_word = word.to_lowercase();
    for anagram in possible_anagrams.iter().filter(|a| a.len() == lc_word.len()) {
        let lc_anagram = anagram.to_lowercase();
        if lc_word == lc_anagram {
            continue;
        }
        let sorted = |word: &str| {
            let mut w = word.chars().collect::<Vec<char>>();
            w.sort_unstable();
            w
        };

        if sorted(&lc_word) == sorted(&lc_anagram) {
            res.insert(*anagram);
        }
    }
    res
}
