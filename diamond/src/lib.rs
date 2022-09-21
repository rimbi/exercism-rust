pub fn get_diamond(c: char) -> Vec<String> {
    let width = 2 * (c as usize - 'A' as usize) + 1;
    let res = ('A'..=c)
        .enumerate()
        .map(|(i, ch)| {
            let mut v = vec![' '; width];
            let middle = width / 2;
            v[middle + i] = ch;
            v[middle - i] = ch;
            String::from_iter(&v)
        })
        .collect::<Vec<String>>();
    res.iter()
        .chain(res.iter().rev().skip(1))
        .cloned()
        .collect::<Vec<_>>()
}
