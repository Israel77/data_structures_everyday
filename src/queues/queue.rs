pub trait Queue<T> {
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Option<T>;
    fn front(&self) -> Option<T>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}
