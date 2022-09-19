use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    iter::Flatten,
};

const ELEMS_SIZE: u64 = 257;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CustomSet<T> {
    elements: Vec<Vec<T>>,
}

pub struct CustomSetIterator<'a, T> {
    iterator: Flatten<std::slice::Iter<'a, Vec<T>>>,
}

impl<'a, T> Iterator for CustomSetIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl<'a, T> IntoIterator for &'a CustomSet<T> {
    type Item = &'a T;

    type IntoIter = CustomSetIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CustomSetIterator {
            iterator: self.elements.iter().flatten(),
        }
    }
}

impl<T: Hash + PartialEq + Copy> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = Self {
            elements: vec![vec![]; ELEMS_SIZE as usize],
        };
        for e in input {
            set.add(*e);
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        let i = self.get_index(element);
        self.elements[i].contains(element)
    }

    pub fn add(&mut self, element: T) {
        let i = self.get_index(&element);
        let v = &mut self.elements[i];
        if !v.contains(&element) {
            v.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for e in self {
            if !other.contains(e) {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.elements.iter().all(|v| v.is_empty())
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut new_set = Self::new(&[]);
        for e in self {
            if other.contains(e) {
                new_set.add(*e);
            }
        }
        new_set
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut new_set = Self::new(&[]);
        for e in self {
            if !other.contains(e) {
                new_set.add(*e);
            }
        }
        new_set
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut new_set = other.clone();
        for e in self {
            new_set.add(*e);
        }
        new_set
    }

    fn get_index(&self, element: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        let x = hasher.finish();
        let i = (x % ELEMS_SIZE) as usize;
        i
    }
}
