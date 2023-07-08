use crate::vm::instruction::{ABC, ABx};
use crate::vm::opcodes::{OpCode, Operation};

mod vm;


fn main() {
    println!("{:?}", OpCode::MOVE.get_option());
    println!("{:?}", OpCode::TESTTEST.get_option());

    let abc = ABx::new(99999903u32);
    println!("code: {}", abc.get_sbx());
}
