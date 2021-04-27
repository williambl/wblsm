pub struct Stack {
    vec: Vec<u32>,
    frames: Vec<StackFrame>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { vec: Vec::new(), frames: Vec::new() }
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

    pub fn enter_frame(&mut self, current_program_pointer: usize) {
        self.frames.push(StackFrame { bound: self.len(), program_pointer: current_program_pointer })
    }

    pub fn exit_frame(&mut self) -> Option<usize> {
        if let Some(frame) = self.frames.pop() {
            while self.len() > frame.bound {
                self.pop();
            }
            return Some(frame.program_pointer)
        }
        return None;
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

}

struct StackFrame {
    bound: usize,
    program_pointer: usize
}