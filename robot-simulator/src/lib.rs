// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Robot(i32, i32, Direction);

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self(x, y, d)
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self(self.0, self.1, self.2.turn_right())
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self(self.0, self.1, self.2.turn_left())
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut robot = self;
        match self.2 {
            Direction::North => robot.1 += 1,
            Direction::East => robot.0 += 1,
            Direction::South => robot.1 -= 1,
            Direction::West => robot.0 -= 1,
        }
        robot
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            robot = match instruction {
                'A' => robot.advance(),
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                _ => panic!("invalid instruction!"),
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
