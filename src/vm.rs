use crate::instructions::Instruction;
use crate::stack::Stack;

pub(crate) struct VM {
    pub(crate) program: Vec<u32>,
    pub(crate) program_pointer: usize,
    pub(crate) stack: Stack,
    pub(crate) heap: [u8; 1024]
}

impl VM {
    pub fn run_program(&mut self) {
        while self.program_pointer < self.program.len() {
            let old_program_pointer = self.program_pointer;
            if let Some(instr) = Instruction::from_opcode(*self.program.get(self.program_pointer).unwrap_or(&0)) {
                instr.run(self)
            }
            if old_program_pointer == self.program_pointer {
                self.program_pointer += 1;
            }
        }
    }

    pub fn enter_frame(&mut self) {
        self.stack.enter_frame(self.program_pointer)
    }

    pub fn jump(&mut self, jump_to: usize) {
        self.program_pointer = jump_to;
    }

    pub fn enter_frame_and_jump(&mut self, jump_to: usize) {
        self.enter_frame();
        self.jump(jump_to);
    }

    pub fn exit_frame(&mut self) {
        if let Some(pointer) = self.stack.exit_frame() {
            self.jump(pointer);
        }
    }
}