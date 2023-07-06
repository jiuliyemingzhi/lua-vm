use crate::vm::opcodes::OpCode;

mod vm;


fn main() {
    println!("{:?}", OpCode::MOVE.get_option());
    println!("{:?}", OpCode::TESTTEST.get_option());
}
