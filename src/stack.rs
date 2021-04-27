pub struct Stack {
    vec: Vec<u32>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { vec: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.vec.pop()
    }

    pub fn push(&mut self, element: u32) {
        self.vec.push(element)
    }

    pub fn peek(&mut self) -> Option<&u32> {
        self.vec.last()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}