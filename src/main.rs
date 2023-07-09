use std::mem::size_of_val;
use std::rc::Rc;
use crate::api::define::{LuaType, OptionLuaType};
use crate::vm::opcodes::{OpCode, Operation};

mod vm;
mod api;
mod state;
mod utils;


fn main() {
    println!("{:?}", OpCode::MOVE.get_option());
    println!("{:?}", OpCode::TESTTEST.get_option());
    println!("{}", size_of_val(&Some(LuaType::String(Rc::new("xxxxxx".to_string())))))
}
