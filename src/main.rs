use std::mem::size_of_val;
use std::rc::Rc;
use lua_vm::api::define::LuaType;
use lua_vm::vm::opcodes::OpCode;

fn main() {
    println!("{:?}", OpCode::MOVE.get_option());
    println!("{:?}", OpCode::TESTTEST.get_option());
    println!("{}", size_of_val(&LuaType::String(Rc::new("xxxxxx".to_string()))));
}
