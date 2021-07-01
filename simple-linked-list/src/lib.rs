use std::iter::FromIterator;

#[derive(PartialEq, Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut node = &self.head;
        let mut i = 0;
        while node.is_some() {
            i += 1;
            node = &node.as_ref().unwrap().next;
        }
        i
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let head = self.head.take();
        let mut head = head.unwrap();
        self.head = head.next.take();
        Some(head.data)
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            return Some(&node.data);
        }
        None
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        while let Some(data) = self.pop() {
            list.push(data);
        }
        list
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        iter.into_iter().for_each(|data| list.push(data.clone()));
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = vec![];
        while let Some(data) = self.pop() {
            v.push(data);
        }
        v.into_iter().rev().collect()
    }
}
