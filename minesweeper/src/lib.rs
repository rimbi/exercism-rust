use std::{cmp::min, collections::HashSet};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let mut res = vec![];
    for i in 0..height {
        let width = minefield.first().unwrap().len();
        let get_neighbours = |i: usize, j: usize| {
            vec![
                (i.saturating_sub(1), j),
                (i.saturating_sub(1), j.saturating_sub(1)),
                (i, j.saturating_sub(1)),
                (min(i + 1, height - 1), j.saturating_sub(1)),
                (min(i + 1, height - 1), j),
                (min(i + 1, height - 1), min(j + 1, width - 1)),
                (i, min(j + 1, width - 1)),
                (i.saturating_sub(1), min(j + 1, width - 1)),
                ]
                .iter()
                .cloned()
                .collect::<HashSet<(usize, usize)>>()
        };

        let new_line: String = minefield[i]
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(j, &c)| {
                if c == '*' as u8 {
                    '*'
                } else {
                    let ns = get_neighbours(i, j);
                    let point = ns.iter().filter(|(i, j)| minefield[*i].as_bytes()[*j] == '*' as u8).count();
                    if point > 0 {
                        (point as u8 + b'0') as char
                    } else {
                        ' '
                    }
                }
            })
            .collect();
        res.push(new_line);
    }
    res
}
