pub fn encode(source: &str) -> String {
    if !is_consistent(source) {
        return source.to_string();
    }
    source
        .chars()
        .fold(vec![], |mut v, c| {
            match v.last_mut() {
                Some((ch, count)) if c == *ch => *count += 1,
                Some(_) | None => v.push((c, 1)),
            }
            v
        })
        .iter()
        .map(|(c, count)| {
            if *count == 1 {
                format!("{}", c)
            } else {
                format!("{}{}", count, c)
            }
        })
        .collect()
}

fn is_consistent(source: &str) -> bool {
    source.to_ascii_uppercase() == source || source.to_ascii_lowercase() == source
}

pub fn decode(source: &str) -> String {
    if !is_consistent(source) {
        return source.to_string();
    }
    source
        .split_inclusive(|c: char| !c.is_ascii_digit())
        .map(|split| {
            let ch = split.chars().last().unwrap();
            let n = split[..split.len() - 1].parse().unwrap_or(1);
            std::iter::repeat(ch).take(n).collect::<String>()
        })
        .collect()
}
