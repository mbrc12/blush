/* A fixed size buffer which is always consumed in full. */

// Maximum number of messages that can be held in the system after any frame.
const BUFFER_CAPACITY: usize = 1000;

type BufferStore<T> = [T; BUFFER_CAPACITY];

pub struct Buffer<T: Default + Clone> {
    data: BufferStore<T>,
    tail: usize
}

impl<T: Default + Clone> Buffer<T> {
    pub fn new() -> Self {
        let data: BufferStore<T> = std::array::from_fn(|_| T::default());
        Buffer { 
            data,
            tail: 0
        }
    }

    pub fn push(&mut self, item: T) {
        assert!(self.tail < BUFFER_CAPACITY);
        self.data[self.tail] = item;
        self.tail += 1;
    }

    fn clear(&mut self) {
        self.tail = 0;
    }

    pub fn items(&mut self) -> Vec<T> {
        self.into()
    }
}

impl<T: Default + Clone> From<&mut Buffer<T>> for Vec<T> {
    fn from(buf: &mut Buffer<T>) -> Self {
        let mut vec = vec![];
        for idx in 0 .. buf.tail {
            vec.push(buf.data[idx].clone());
        }
        buf.clear();
        vec
    }
}

impl<T: Default + Clone> Default for Buffer<T> {
    fn default() -> Self {
        Buffer::<T>::new()
    }
}

#[cfg(test)]

mod queue_tests {
    use super::Buffer;

    #[test]
    fn one() {
        let mut buf = Buffer::<i32>::default();
        for i in 1..10 {
            buf.push(i);
        }
        let mut sum = 0;
        for item in buf.items() {
            sum += item;
        }
        assert_eq!(sum, 45);
    }
}
