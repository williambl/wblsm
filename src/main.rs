use crate::stack::Stack;
use crate::instructions::Instruction;
use std::io;

mod stack;
mod instructions;

struct VmData {
    program: Vec<u32>,
    program_pointer: usize,
    stack: Stack,
    heap: [u8; 1024]
}

fn main() {
    let mut vm: VmData = VmData {
        program: read_input(),
        program_pointer: 0,
        stack: Stack::new(),
        heap: [0; 1024]
    };

    for opcode in vm.program {
        if let Some(mut instr) = Instruction::from_opcode(opcode) {
            instr.run(&mut (vm.stack))
        }
    }
    println!("{}", vm.stack.peek().unwrap_or(&0))
}

fn read_input() -> Vec<u32> {
    let stdin = io::stdin();
    let mut program: Vec<u32> = Vec::new();

    let mut line = String::new();
    while let Ok(_) = stdin.read_line(&mut line)  {
        line.pop();
        if line.is_empty() { break }

        if let Ok(value) = line.parse::<u32>() {
            program.push(value);
        }
        line.clear();
    }

    program
}