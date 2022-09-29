use std::fmt::Debug;

#[derive(Default)]
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    read_ptr: usize,
    write_ptr: usize,
    is_full: bool,
    is_empty: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default + Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![T::default(); capacity],
            is_empty: true,
            ..Default::default()
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full {
            return Err(Error::FullBuffer);
        }
        self.overwrite(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty {
            return Err(Error::EmptyBuffer);
        }
        let element = self.buffer[self.read_ptr].clone();
        self.read_ptr = (self.read_ptr + 1) % self.capacity();
        self.is_empty = self.read_ptr == self.write_ptr;
        self.is_full = false;
        Ok(element)
    }

    pub fn clear(&mut self) {
        *self = Self::new(self.capacity());
    }

    pub fn overwrite(&mut self, element: T) {
        self.buffer[self.write_ptr] = element;
        self.write_ptr = (self.write_ptr + 1) % self.capacity();
        if self.is_full {
            self.read_ptr = self.write_ptr;
        }
        self.is_full = self.read_ptr == self.write_ptr;
        self.is_empty = false;
    }

    fn capacity(&self) -> usize {
        self.buffer.len()
    }
}
