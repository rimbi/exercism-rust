pub struct PascalsTriangle(u32);

type Row = Vec<u32>;
type Rows = Vec<Row>;

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Rows {
        let mut res: Rows = vec![];
        for row in 0..self.0 {
            let mut cur_row = vec![];
            for i in 0..=row {
                if i == 0 || i == row {
                    cur_row.push(1);
                } else if let Some(last_row) = res.last() {
                    let value =
                        last_row.get((i - 1) as usize).unwrap() + last_row.get(i as usize).unwrap();
                    cur_row.push(value);
                }
            }
            res.push(cur_row);
        }
        res
    }
}
