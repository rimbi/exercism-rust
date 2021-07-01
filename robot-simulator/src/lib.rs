// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<i32> for Direction {
    fn from(val: i32) -> Self {
        match (val + 4) % 4 {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unreachable!(),
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let mut r = self;
        r.d = (r.d as i32 + 1).into();
        r
    }

    pub fn turn_left(self) -> Self {
        let mut r = self;
        r.d = (r.d as i32 - 1).into();
        r
    }

    pub fn advance(self) -> Self {
        let mut r = self;
        match r.d {
            Direction::North => r.y += 1,
            Direction::South => r.y -= 1,
            Direction::East => r.x += 1,
            Direction::West => r.x -= 1,
        }
        r
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .to_ascii_uppercase()
            .chars()
            .fold(self, |r, c| match c {
                'L' => r.turn_left(),
                'R' => r.turn_right(),
                'A' => r.advance(),
                _ => r,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
