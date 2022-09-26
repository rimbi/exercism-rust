use std::collections::HashMap;

pub fn rotate(input: &str, key: i8) -> String {
    let mut src_lower = ('a'..='z').collect::<Vec<_>>();
    let mut src_upper = ('A'..='Z').collect::<Vec<_>>();
    let mut dst_lower = src_lower.clone();
    let mut dst_upper = src_upper.clone();
    src_upper.append(&mut src_lower);
    if key > 0 {
        dst_lower.rotate_left(key as usize);
        dst_upper.rotate_left(key as usize);
    } else {
        dst_lower.rotate_right(key.unsigned_abs() as usize);
        dst_upper.rotate_right(key.unsigned_abs() as usize);
    }
    dst_upper.append(&mut dst_lower);
    let map = src_upper.iter().zip(&dst_upper).collect::<HashMap<_, _>>();
    input
        .chars()
        .map(|c| if c.is_alphabetic() { *map[&c] } else { c })
        .collect()
}
