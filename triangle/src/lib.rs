use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Sum;
use std::ops::Add;
pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: 'static + Copy + Default + Hash + Eq + PartialOrd + Sum<T> + Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().filter(|&s| *s > T::default()).count() != 3 {
            return None;
        }
        let sum: T = sides.iter().cloned().sum();
        if sides.iter().any(|&side| sum < side + side) {
            return None;
        }
        Some(Triangle { sides })
    }

    fn get_number_of_uniqu_sides(&self) -> usize {
        self.sides.iter().cloned().collect::<HashSet<T>>().len()
    }

    pub fn is_equilateral(&self) -> bool {
        self.get_number_of_uniqu_sides() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.get_number_of_uniqu_sides() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.get_number_of_uniqu_sides() == 2
    }
}
