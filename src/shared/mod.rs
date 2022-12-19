#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Stack { elements }
    }

    pub fn pop(&mut self) -> T {
        self.elements.remove(self.elements.len() - 1)
    }

    pub fn push(&mut self, char: T) -> () {
        self.elements.push(char)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.len() > 0
    }
}
