// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::{BTreeMap};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut m_words = magazine
        .iter()
        .fold(BTreeMap::new(), |mut acc, w| {
            *acc.entry(w).or_default() += 1;
            acc
        });
    for w in note {
        let entry: &mut u8 = m_words.entry(w).or_default();
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }
    true
}
