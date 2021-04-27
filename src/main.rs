use crate::stack::Stack;
use std::io;
use std::io::Read;
use std::str::FromStr;

mod stack;

fn main() {
    let mut stack: Stack = Stack::new();
    let mut program: Vec<u32> = read_input();

    while !program.is_empty() {
        if let Some(x) = program.pop() {
            println!("{}", x)
        }
    }
}

fn read_input() -> Vec<u32> {
    let mut stdin = io::stdin();
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