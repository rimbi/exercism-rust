use std::vec;

#[derive(Default)]
pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut v = vec![vec![1]];
        (1..self.row_count as usize).for_each(|row| {
            let r = (0..(row + 1) as usize)
                .map(|i| match i {
                    0 => 1,
                    _ if i == row => 1,
                    _ => v[row - 1][i - 1] + v[row - 1][i],
                })
                .collect();
            v.push(r);
        });
        v
    }
}
