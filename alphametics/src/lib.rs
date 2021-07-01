use std::collections::{BTreeMap, BTreeSet, HashMap};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut input = input
        .trim()
        .split("==")
        .map(|w| w.trim())
        .collect::<Vec<&str>>();
    // println!("{:?}", input);
    if input.len() != 2 {
        return None;
    }
    let result = input.pop().unwrap();
    // println!("{:?} {}", input, result);
    let input = input
        .first()
        .unwrap()
        .split("+")
        .map(|w| w.trim())
        .collect::<Vec<&str>>();
    if input.len() != 2 {
        return None;
    }
    let (first, second) = (input[0], input[1]);
    println!("{} {} {}", first, second, result);
    let set: BTreeSet<char> = first.chars().chain(second.chars()).collect();
    // let map: BTreeMap<&char, u8> = set.iter().enumerate().map(|(i, v)| (v, i as u8)).collect();
    None
}
