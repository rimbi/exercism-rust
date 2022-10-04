use std::{marker::PhantomData, ptr::NonNull};

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

type NodePtr<T> = Option<NonNull<Node<T>>>;

pub struct LinkedList<T> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

unsafe impl<T> Send for LinkedList<T> {}
unsafe impl<T> Sync for LinkedList<T> {}
// impl<T> Display for LinkedList<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut cur = self.head;
//         while cur != std::ptr::null() {
//             if let Some(cur_2) = &cur_1.borrow().prev {
//                 writeln!(f, "counter = {}", Rc::strong_count(&cur_2))?;
//             }
//             cur = cur_1.borrow().next.clone();
//         }
//         Ok(())
//     }
// }

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Node<T> {
    next: NodePtr<T>,
    prev: NodePtr<T>,
    val: T,
}

pub struct Cursor<'a, T> {
    cur: NodePtr<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    cur: NodePtr<T>,
    _marker: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let mut ptr = self.head;
        let mut len = 0;
        while ptr.is_some() {
            len += 1;
            unsafe {
                ptr = ptr.unwrap().as_ref().next;
            }
        }
        len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: self.head,
            _marker: PhantomData,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut self.cur?.as_mut().val) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur = self.cur?.as_ref().next;
        }
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur = self.cur?.as_ref().prev;
        }
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let cur = self.cur?;
        unsafe {
            let prev = cur.as_ref().prev;
            let next = cur.as_ref().next;
            if let Some(mut next) = next {
                next.as_mut().prev = prev;
            } else {
                self.list.tail = prev;
            }
            if let Some(mut prev) = prev {
                prev.as_mut().next = next;
            } else {
                self.list.head = next;
            }

            let node = Box::from_raw(cur.as_ptr());
            self.cur = if next.is_some() { next } else { prev };
            Some(node.val)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            let cur = self.cur;
            let node = if let Some(cur) = cur {
                Node {
                    val: element,
                    next: cur.as_ref().next,
                    prev: Some(cur),
                }
            } else {
                Node {
                    val: element,
                    next: None,
                    prev: None,
                }
            };
            let node = NonNull::new(Box::into_raw(Box::new(node)));
            if let Some(mut cur) = cur {
                if let Some(mut next) = cur.as_ref().next {
                    next.as_mut().prev = node;
                }
                cur.as_mut().next = node;
            }
            if self.list.head.is_none() {
                self.list.head = node;
                self.list.tail = node;
                self.cur = self.list.head;
            } else if self.list.tail.unwrap().as_ref().next.is_some() {
                self.list.tail = node;
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            let cur = self.cur;
            let new_node = if let Some(cur) = cur {
                Node {
                    val: element,
                    next: Some(cur),
                    prev: cur.as_ref().prev,
                }
            } else {
                Node {
                    val: element,
                    next: None,
                    prev: None,
                }
            };
            let new_node = NonNull::new(Box::into_raw(Box::new(new_node)));
            if let Some(mut cur) = cur {
                if let Some(mut prev) = cur.as_ref().prev {
                    prev.as_mut().next = new_node;
                }
                cur.as_mut().prev = new_node;
            }
            if self.list.head.is_none() {
                self.list.head = new_node;
                self.list.tail = new_node;
                self.cur = self.list.head;
            } else if new_node.unwrap().as_ref().prev.is_none() {
                self.list.head = new_node;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let cur = self.cur?;
        unsafe {
            let prev = cur;
            self.cur = cur.as_ref().next;
            Some(&prev.as_ref().val)
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
