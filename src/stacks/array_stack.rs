use super::stack::Stack;

const INITIAL_CAPACITY: usize = 2048;
pub struct ArrayStack<T: Copy> {
    array: Box<[Option<T>]>,
    stack_pointer: usize,
}

impl<T: Copy> ArrayStack<T> {
    pub fn new() -> Self {
        Self {
            array: Box::from([None; INITIAL_CAPACITY]),
            stack_pointer: 0,
        }
    }

    fn resize(&mut self, new_capacity: usize) {
        if new_capacity < self.size() {
            panic!("Cannot shrink stack!")
        }

        let mut new_array = vec![None; self.array.len()].into_boxed_slice();
        for i in 0..self.size() {
            new_array[i] = self.array[i];
        }
        self.array = new_array;
    }

    fn capacity(&self) -> usize {
        self.array.len()
    }
}

impl<T: Copy> Stack<T> for ArrayStack<T> {
    fn push(&mut self, value: T) {
        if self.size() < self.capacity() {
            self.array[self.stack_pointer] = Some(value);
            self.stack_pointer += 1;
        } else {
            self.resize(self.capacity() * 2);
            self.push(value);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.size() == 0 {
            return None;
        }

        self.stack_pointer -= 1;
        self.array[self.stack_pointer]
    }

    fn peek(&self) -> Option<&T> {
        if self.size() == 0 {
            None
        } else {
            self.array[self.stack_pointer - 1].as_ref()
        }
    }

    fn size(&self) -> usize {
        self.stack_pointer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = ArrayStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = ArrayStack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn test_is_empty() {
        let mut stack = ArrayStack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_size() {
        let mut stack = ArrayStack::new();
        assert_eq!(stack.size(), 0);
        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        stack.pop();
        assert_eq!(stack.size(), 1);
    }
}
