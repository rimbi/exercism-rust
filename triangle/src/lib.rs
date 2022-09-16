use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Add<Output = T> + PartialOrd + Copy + Default> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides[0] > sides[1] + sides[2]
            || sides[1] > sides[0] + sides[2]
            || sides[2] > sides[0] + sides[1]
            || sides.iter().any(|s| *s == T::default())
        {
            return None;
        }
        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[1] != self.sides[2]
            && self.sides[0] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }
}
