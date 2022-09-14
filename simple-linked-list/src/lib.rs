use std::iter::FromIterator;

type NextNode<T> = Box<Node<T>>;

struct Node<T> {
    next: Option<NextNode<T>>,
    data: T,
}

pub struct SimpleLinkedList<T> {
    head: Option<NextNode<T>>,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
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
        std::iter::successors(self.head.as_ref(), |cur| cur.next.as_ref()).count()
    }

    pub fn push(&mut self, element: T) {
        let node = Node {
            next: self.head.take(),
            data: element,
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.as_ref()?;
        let res = self.head.take().unwrap();
        self.head = res.next;
        Some(res.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|val| &val.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self { head: None };
        let mut s = self;
        while let Some(data) = s.pop() {
            list.push(data);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self { head: None };
        for el in iter {
            list.push(el);
        }
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

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut res = vec![];
        while let Some(data) = _linked_list.pop() {
            res.push(data);
        }
        res.reverse();
        res
    }
}
