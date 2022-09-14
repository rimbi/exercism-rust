pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }
    if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }
    let bottles = |n: u32| {
        let mut res = format!("{} bottle", n);
        if n > 1 {
            res.push_str("s");
        }
        res
    };
    let b1 = bottles(n);
    let b2 = bottles(n - 1);
    format!("{b1} of beer on the wall, {b1} of beer.\nTake one down and pass it around, {b2} of beer on the wall.\n", b1=b1, b2=b2)
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
