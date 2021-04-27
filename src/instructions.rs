use crate::instructions::Instruction::*;
use crate::stack::Stack;
use std::ops::Shl;

enum Instruction {
    Load(u16),
    Write(u16),
    LoadConst(u16),
    Duplicate(u16),

    Jump(u16),
    Call(u16),
    Return,
    Panic,

    AddInt,
    SubtractInt,
    MultiplyInt,
    DivideInt,
    ModuloInt,

    AddFloat,
    SubtractFloat,
    MultiplyFloat,
    DivideFloat,
    ModuloFloat
}

impl Instruction {
    pub fn run(&mut self, stack: &mut Stack) {
        match self {
            LoadConst(value) => stack.push(*value as u32),
            Duplicate(downwardsBy) => {
                if let Some(originalTopValue) = stack.pop() {
                    let mut temp_stack = Stack::new();
                    for _ in 1..downwardsBy {
                        temp_stack.push(stack.pop().unwrap_or_default());
                    }
                    stack.push(originalTopValue);
                    while !temp_stack.is_empty() {
                        stack.push(temp_stack.pop().unwrap_or_default());
                    }
                    stack.push(originalTopValue);
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
                        stack.push(f32::from_bits(a)+f32::from_bits(b).to_bits())
                    }
                }
            }
            SubtractFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(f32::from_bits(a)-f32::from_bits(b).to_bits())
                    }
                }
            }
            MultiplyFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(f32::from_bits(a)*f32::from_bits(b).to_bits())
                    }
                }
            }
            DivideFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(f32::from_bits(a)/f32::from_bits(b).to_bits())
                    }
                }
            }
            ModuloFloat => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(f32::from_bits(a)%f32::from_bits(b).to_bits())
                    }
                }
            }
            _ => {}
        }
    }

    pub fn from_packed_instruction(packed: u32) -> Option<Instruction> {
        let opcode = packed >> 24;
        match opcode {
            0 => Some(Load(u16_operand_from_packed(packed))),
            1 => Some(Write(u16_operand_from_packed(packed))),
            2 => Some(LoadConst(u16_operand_from_packed(packed))),
            3 => Some(Duplicate(u16_operand_from_packed(packed))),
            4 => Some(Jump(u16_operand_from_packed(packed))),
            5 => Some(Call(u16_operand_from_packed(packed))),
            6 => Some(Return),
            7 => Some(Panic),
            8 => Some(AddInt),
            9 => Some(SubtractInt),
            10 => Some(MultiplyInt),
            11 => Some(DivideInt),
            12 => Some(ModuloInt),
            13 => Some(AddFloat),
            14 => Some(SubtractFloat),
            15 => Some(MultiplyFloat),
            16 => Some(DivideFloat),
            17 => Some(ModuloFloat),
            _ => None
        }
    }
}

fn operands_from_packed(packed: u32) -> u32 {
    packed & 0xFF000000
}

fn u16_operand_from_packed(packed: u32) -> u16 {
    (operands_from_packed(packed) >> 8) as u16
}
