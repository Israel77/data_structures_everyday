use std::{cell::RefCell, rc::Rc};

use super::queue::Queue;

type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: NodeLink<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }

    fn into_link(self) -> NodeLink<T> {
        Some(Rc::new(RefCell::from(self)))
    }
}
pub struct LinkedQueue<T> {
    front: NodeLink<T>,
    rear: NodeLink<T>,
    size: usize,
}

impl<T> LinkedQueue<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            rear: None,
            size: 0,
        }
    }
}

impl<T: Clone> Queue<T> for LinkedQueue<T> {
    fn enqueue(&mut self, value: T) {
        let new_link = Node::new(value).into_link();

        if let Some(rear_link) = self.rear.clone() {
            // Makes the previous rear node point to the new
            rear_link.borrow_mut().next = new_link.clone();
            self.rear = new_link;
        } else {
            assert!(self.front.is_none());
            self.front = new_link.clone();
            self.rear = new_link;
        }

        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.front.is_none() {
            assert!(self.rear.is_none());
            return None;
        }

        self.front.take().map(|prev_front| {
            self.front = RefCell::clone(&prev_front).into_inner().next;
            self.size -= 1;
            if self.size == 0 {
                self.rear = None
            }
            RefCell::clone(&prev_front).into_inner().value
        })
    }

    fn front(&self) -> Option<T> {
        self.front
            .as_ref()
            .map(|front| RefCell::clone(front).into_inner().value)
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_and_size() {
        let mut queue = LinkedQueue::new();
        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.size(), 3);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_dequeue() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.size(), 2);

        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.size(), 1);

        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());

        // Test dequeue on an empty queue
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_front() {
        let mut queue = LinkedQueue::new();
        assert_eq!(queue.front(), None);

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.front(), Some(10));
        assert_eq!(queue.size(), 3);

        // Front should remain the same after dequeue until the front element is removed
        queue.dequeue();
        assert_eq!(queue.front(), Some(20));
    }

    #[test]
    fn test_mixed_operations() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        assert_eq!(queue.front(), Some(10));

        queue.dequeue();
        queue.enqueue(30);
        queue.enqueue(40);
        assert_eq!(queue.front(), Some(20));

        queue.dequeue();
        assert_eq!(queue.front(), Some(30));
        assert_eq!(queue.size(), 2);
    }
}
