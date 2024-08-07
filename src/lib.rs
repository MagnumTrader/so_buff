//! The buffest buffer,
use std::mem::MaybeUninit;

type Result<T, R> = std::result::Result<T, Error<R>>;

pub struct Buffer<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    /// the current amount of items in the buffer
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
        // SAFETY: Filling with unititialized data is Safe (i guess?)
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
        unsafe {
            let slice: *mut [T] =
                std::ptr::slice_from_raw_parts_mut(self.data.as_mut_ptr().cast::<T>(), self.len);
            slice.drop_in_place();
        }
    }
}

impl<T, const N: usize> IntoIterator for Buffer<T, N>{
    type Item = T;

    type IntoIter = IntoIter<T, N>;

    /// Consumes the buffer and turns it into an Iterator
    /// that can be used to consume the containing items.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {buffer: self, current_index: 0}
    }
}

pub struct IntoIter<T, const N: usize> {
    buffer: Buffer<T, N>,
    current_index: usize,
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.buffer.len {
            return None;
        }

        let value = std::mem::replace(
            &mut self.buffer.data[self.current_index],
            MaybeUninit::uninit(),
        );

        self.current_index += 1;

        // SAFETY: current_index is checked to be < len.
        // where len indicate last item that contains a T.
        Some(unsafe { value.assume_init() })
    }
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        let buf = std::mem::replace(&mut self.buffer, Buffer::new());
        drop(buf);
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
