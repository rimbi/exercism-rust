pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (row, v) in input.iter().enumerate() {
        if let Some(&max) = v.iter().max() {
            for column in v
                .iter()
                .enumerate()
                .filter(|(_, &v)| v == max)
                .map(|(i, _)| i)
            {
                if max == input.iter().map(|v| v[column]).min().unwrap() {
                    res.push((row, column))
                }
            }
        }
    }
    res
}
