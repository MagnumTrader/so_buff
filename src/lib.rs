//! The buffest buffer,

use std::{mem::{self, MaybeUninit}, slice::from_raw_parts_mut};

type Result<T, R> = std::result::Result<T, Error<R>>;

pub struct Buffer<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    /// the current amount of items in the buffer 0..self.len is initialized memory
    len: usize,
}

impl<T, const N: usize> Buffer<T, N> {
    /// Returning a new instance of a buffer.
    /// Need to specify type and size.
    ///
    ///```rust
    /// // creates a buffer of i32s with a capacity of 10
    /// # use so_buff::Buffer;
    /// let mut buf: Buffer<i32, 10> = Buffer::new();
    ///
    /// buf.push(1);
    /// buf.push(2);
    /// buf.push(3);
    ///```
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let data = [const {MaybeUninit::uninit()}; N];
        Self { data, len: 0 }
    }

    /// Pushes a value into the buffer.
    /// Will return an error containing the value if
    /// the caller tried to push when the buffer is full.
    ///
    ///```rust
    /// # use so_buff::{Buffer, Error};
    /// let mut buf = Buffer::<i32, 3>::new();
    ///
    /// let _ = buf.push(1);
    /// let _ = buf.push(2);
    /// let _ = buf.push(3);
    /// let should_fail = buf.push(4);
    ///
    /// assert_eq!(Err(Error::BufferIsFull(4)), should_fail);
    ///```
    #[must_use = "May fail if there is no space left"]
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(Error::BufferIsFull(value));
        }

        self.data[self.len].write(value);
        self.len += 1;

        Ok(())
    }

}

impl<T, const N: usize> Drop for Buffer<T, N> {
    fn drop(&mut self) {
        let slice = std::ptr::slice_from_raw_parts_mut(self.data.as_mut_ptr().cast::<T>(), self.len);
        unsafe {slice.drop_in_place()};
    }
}

impl<T, const N: usize> IntoIterator for Buffer<T, N>{
    type Item = T;

    type IntoIter = IntoIter<T, N>;

    /// Consumes the buffer and turns it into an Iterator
    /// that can be used to consume the containing items.
    fn into_iter(mut self) -> Self::IntoIter {

        // Setting the buffer len to 0 so that when the data gets
        // dropped, buffer will not run drop on any items,
        // since they will be uninitialized.
        let len = self.len;
        self.len = 0;

        let buffer = mem::replace(&mut self.data, [const {MaybeUninit::uninit()}; N]);
        IntoIter {buffer, len, current_index: 0}
    }
}


pub struct IntoIter<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    len: usize,
    current_index: usize,
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.len {
            return None;
        }

        let value = std::mem::replace(
            &mut self.buffer[self.current_index],
            MaybeUninit::uninit(),
        );

        self.current_index += 1;

        // SAFETY: current_index is checked to be < len before assigning to value.
        // where current_index is the first item that contains a T
        // and len indicate the last item that contains a T.
        // therefore it's safe to assume_init().
        Some(unsafe { value.assume_init() })
    }
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        for index in self.current_index..self.len {
            unsafe {self.buffer[index].assume_init_drop();}
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
#[non_exhaustive]
pub enum Error<T> {
    BufferIsFull(T),
}

impl<T: std::fmt::Debug> std::error::Error for Error<T> {}

impl<T: std::fmt::Debug> std::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn push() {
        let mut buffer: Buffer<i32, 3> = Buffer::new();
        assert_eq!(Ok(()), buffer.push(1));
        assert_eq!(Ok(()), buffer.push(2));
        assert_eq!(Ok(()), buffer.push(3));
    }

    #[test]
    fn push_to_full() {
        let mut buffer: Buffer<i32, 2> = Buffer::new();
        assert_eq!(Ok(()), buffer.push(1));
        assert_eq!(Ok(()), buffer.push(2));
        assert_eq!(Err(Error::BufferIsFull(3)), buffer.push(3));
    }

    #[test]
    fn push_and_iter() {
        let mut buffer: Buffer<i32, 3> = Buffer::new();
        let _ = buffer.push(1);
        let _ = buffer.push(2);
        let _ = buffer.push(3);

        let mut buf_iter = buffer.into_iter();

        assert_eq!(Some(1), buf_iter.next());
        assert_eq!(Some(2), buf_iter.next());
        assert_eq!(Some(3), buf_iter.next());
        assert_eq!(None, buf_iter.next());
    }
}
