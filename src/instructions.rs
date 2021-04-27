use crate::instructions::Instruction::*;
use crate::stack::Stack;

pub(crate) enum Instruction {
    NoOp,               // NO-OP
    Load,               // load value onto stack from heap pointer
    Write,              // write to heap at pointer from stack
    LoadConst0,         // load `0` onto stack
    LoadConst1,         // load `1` onto stack
    Duplicate,          // duplicate top stack element `arg` elements down
    Pop,                // Pops an entry off the stack and does nothing with it.

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
            },
            Pop => {
                stack.pop();
            },
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
            0 => Some(NoOp),
            1 => Some(Load),
            2 => Some(Write),
            3 => Some(LoadConst0),
            4 => Some(LoadConst1),
            5 => Some(Duplicate),
            6 => Some(Pop),
            7 => Some(Jump),
            8 => Some(Call),
            9 => Some(Return),
            10 => Some(Panic),
            11 => Some(AddInt),
            12 => Some(SubtractInt),
            13 => Some(MultiplyInt),
            14 => Some(DivideInt),
            15 => Some(ModuloInt),
            16 => Some(AddFloat),
            17 => Some(SubtractFloat),
            18 => Some(MultiplyFloat),
            19 => Some(DivideFloat),
            20 => Some(ModuloFloat),
            _ => None
        }
    }
}
