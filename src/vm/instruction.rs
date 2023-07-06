use crate::vm::opcodes::{OpCode, Operation};

pub trait Instruction {
    fn code(self) -> OpCode;
}

// impl Instruction for u32 {
//     fn code(self) -> &'static Operation {
//         (self & 0x3F).
//     }
// }