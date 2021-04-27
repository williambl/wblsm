use crate::stack::Stack;
use crate::instructions::Instruction;
use crate::vm::VM;
use std::io;
use bytebuffer::ByteBuffer;

mod stack;
mod instructions;
mod vm;

fn main() {
    let mut vm: VM = VM {
        program: read_input(),
        program_pointer: 0,
        stack: Stack::new(),
        heap: ByteBuffer::new()
    };

    vm.heap.write_u32(4);

    vm.run_program();

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