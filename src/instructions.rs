use crate::instructions::Instruction::*;
use crate::stack::Stack;
use std::ops::Shl;
use std::fmt::{Display, Formatter};

pub(crate) enum Instruction {
    Load,               // load value onto stack from heap pointer
    Write,              // write to heap at pointer from stack
    LoadConst0,         // load `0` onto stack
    LoadConst1,         // load `1` onto stack
    Duplicate,          // duplicate top stack element `arg` elements down

    Jump,               // jump to program pointer
    Call,               // jump to program pointer and open a new frame
    Return,             // step out of a frame
    Panic,              // terminate the program

    // Integer Arithmetic
    AddInt,             // a + b
    SubtractInt,        // a - b
    MultiplyInt,        // a * b
    DivideInt,          // a / b
    ModuloInt,          // a % b

    // Float Arithmetic
    AddFloat,           // a + b
    SubtractFloat,      // a - b
    MultiplyFloat,      // a * b
    DivideFloat,        // a / b
    ModuloFloat         // a % b
}

impl Instruction {
    pub fn run(&mut self, stack: &mut Stack) {
        match self {
            LoadConst0 => stack.push(0),
            LoadConst1 => stack.push(1),
            Duplicate => {
                if let Some(downwards_by) = stack.pop() {
                    if let Some(original_top_value) = stack.pop() {
                        let mut temp_stack = Stack::new();
                        for _ in 0..downwards_by {
                            temp_stack.push(stack.pop().unwrap_or_default());
                        }
                        stack.push(original_top_value);
                        while !temp_stack.is_empty() {
                            stack.push(temp_stack.pop().unwrap_or_default());
                        }
                        stack.push(original_top_value);
                    }
                }
            }
            Panic => {
                panic!()
            }
            AddInt => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a+b)
                    }
                }
            }
            SubtractInt => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a-b)
                    }
                }
            }
            MultiplyInt => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a*b)
                    }
                }
            }
            DivideInt => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a/b)
                    }
                }
            }
            ModuloInt => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a%b)
                    }
                }
            }
            AddFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((f32::from_bits(a)+f32::from_bits(b)).to_bits())
                    }
                }
            }
            SubtractFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((f32::from_bits(a)-f32::from_bits(b)).to_bits())
                    }
                }
            }
            MultiplyFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((f32::from_bits(a)*f32::from_bits(b)).to_bits())
                    }
                }
            }
            DivideFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((f32::from_bits(a)/f32::from_bits(b)).to_bits())
                    }
                }
            }
            ModuloFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((f32::from_bits(a)%f32::from_bits(b)).to_bits())
                    }
                }
            }
            _ => {}
        }
    }

    pub fn from_opcode(opcode: u32) -> Option<Instruction> {
        match opcode {
            0 => Some(Load),
            1 => Some(Write),
            2 => Some(LoadConst0),
            3 => Some(LoadConst1),
            4 => Some(Duplicate),
            5 => Some(Jump),
            6 => Some(Call),
            7 => Some(Return),
            8 => Some(Panic),
            9 => Some(AddInt),
            10 => Some(SubtractInt),
            11 => Some(MultiplyInt),
            12 => Some(DivideInt),
            13 => Some(ModuloInt),
            14 => Some(AddFloat),
            15 => Some(SubtractFloat),
            16 => Some(MultiplyFloat),
            17 => Some(DivideFloat),
            18 => Some(ModuloFloat),
            _ => None
        }
    }
}
