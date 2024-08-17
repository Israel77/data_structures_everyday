pub trait Stack<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}
