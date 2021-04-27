use crate::stack::Stack;
use crate::instructions::Instruction;
use std::io;

mod stack;
mod instructions;

fn main() {
    let mut stack: Stack = Stack::new();
    let mut program: Vec<u32> = read_input();

    for opcode in program {
        if let Some(mut instr) = Instruction::from_opcode(opcode) {
            instr.run(&mut stack)
        }
    }
    println!("{}", stack.peek().unwrap_or(&0))
}

fn read_input() -> Vec<u32> {
    let stdin = io::stdin();
    let mut program: Vec<u32> = Vec::new();

    let mut line = String::new();
    while let Ok(x) = stdin.read_line(&mut line)  {
        line.pop();
        if line.is_empty() { break }

        if let Ok(value) = line.parse::<u32>() {
            program.push(value);
        }
        line.clear();
    }

    program
}