// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub struct NumberIterator {
    lines: Vec<Vec<char>>,
    row: usize,
    column: usize,
    row_count: usize,
    col_count: usize,
}

impl NumberIterator {
    pub fn new(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let row_count = lines.len();
        let col_count = lines[0].len();
        Self {
            lines,
            row: 0,
            column: 0,
            row_count,
            col_count,
        }
    }
}

const CHAR_WIDTH: usize = 3;
const CHAR_HEIGHT: usize = 4;

impl Iterator for NumberIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column == self.col_count && self.row == self.row_count {
            return None;
        }
        let next: String = self.lines[self.row..self.row + CHAR_HEIGHT]
            .iter()
            .flat_map(|l| &l[self.column..self.column + CHAR_WIDTH])
            .cloned()
            .collect();
        self.column += CHAR_WIDTH;
        if self.column == self.col_count {
            self.row += CHAR_HEIGHT;
            if self.row != self.row_count {
                self.column = 0;
            }
        }
        let next = match next.as_str() {
            " _ | ||_|   " => '0',
            "     |  |   " => '1',
            " _  _||_    " => '2',
            " _  _| _|   " => '3',
            "   |_|  |   " => '4',
            " _ |_  _|   " => '5',
            " _ |_ |_|   " => '6',
            " _   |  |   " => '7',
            " _ |_||_|   " => '8',
            " _ |_| _|   " => '9',
            _ => '?',
        };
        Some(next)
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let row_count = input.lines().count();
    if row_count % CHAR_HEIGHT != 0 {
        return Err(Error::InvalidRowCount(row_count));
    }
    if let Some(col_count) = input
        .lines()
        .map(|l| l.len())
        .skip_while(|l| l % CHAR_WIDTH == 0)
        .next()
    {
        return Err(Error::InvalidColumnCount(col_count));
    }
    let iter = NumberIterator::new(input);
    let chars_per_line = input.lines().next().unwrap().len() / CHAR_WIDTH;
    let nums = iter
        .collect::<Vec<_>>()
        .chunks(chars_per_line)
        .map(String::from_iter)
        .collect::<Vec<_>>()
        .join(",");
    Ok(nums)
}
