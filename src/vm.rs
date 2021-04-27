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
        for opcode in program {
            if let Some(mut instr) = Instruction::from_opcode(opcode) {
                instr.run(&mut stack)
            }
        }
    }
}