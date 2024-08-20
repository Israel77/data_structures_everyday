use super::queue::Queue;

pub struct ArrayQueue<T: Clone> {
    buffer: Box<[Option<T>]>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T: Clone> ArrayQueue<T> {
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 2048;
        Self::with_capacity(INITIAL_CAPACITY)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity].into_boxed_slice(),
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    fn capacity(&self) -> usize {
        self.buffer.len()
    }

    fn resize_buffer(&mut self, new_capacity: usize) {
        let mut new_buffer = vec![None; new_capacity].into_boxed_slice();
        for (i, value) in self.buffer.into_iter().enumerate() {
            let index = (i + self.capacity()) % self.capacity();
            new_buffer[index] = value.to_owned();
        }

        self.head = 0;
        self.tail = self.capacity();

        self.buffer = new_buffer;
    }
}

impl<T: Clone> Default for ArrayQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Queue<T> for ArrayQueue<T> {
    fn enqueue(&mut self, value: T) {
        if let Some(_) = self.buffer[self.tail] {
            self.resize_buffer(self.capacity() * 2);
        }

        self.buffer[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.capacity();
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        self.buffer[self.head].take().map(|value| {
            self.head = (self.head + 1) % self.capacity();
            self.size -= 1;

            value
        })
    }

    fn front(&self) -> Option<&T> {
        self.buffer[self.head].as_ref()
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod array_queue_tests {
    use super::*;

    #[test]
    fn test_enqueue_and_dequeue() {
        let mut queue = ArrayQueue::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.size(), 3);
        assert_eq!(queue.front(), Some(&1));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_circular_behavior() {
        let mut queue = ArrayQueue::with_capacity(3);

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        queue.enqueue(4);

        assert_eq!(queue.front(), Some(&2));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_growing_behavior() {
        let mut queue = ArrayQueue::with_capacity(3);

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.size(), 3);
        assert_eq!(queue.capacity(), 3);

        queue.enqueue(4);
        queue.enqueue(5);
        queue.enqueue(6);

        assert!(queue.capacity() > 3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));

        assert!(queue.is_empty());
    }
}
