use std::mem::size_of_val;
use std::ops::{Shl};
use std::rc::Rc;
use lua_vm::api::define::LuaType;
use lua_vm::number::math::TNumber;
use lua_vm::vm::opcodes::OpCode;

fn main() {
    println!("{:?}", OpCode::MOVE.get_option());
    println!("{:?}", OpCode::TESTTEST.get_option());
    println!("{}", size_of_val(&LuaType::String(Rc::new("xxxxxx".to_string()))));
    println!("{:?}", (TNumber::Int(3) / TNumber::Float(1.1f64)).floor_float());
    println!("{:?}", TNumber::Int(3) / TNumber::Float(f64::INFINITY));
    println!("{:?}", TNumber::Float(3.1) % TNumber::Float(1.0));
    println!("{:?}", TNumber::Int(1) << TNumber::Int(1));
    println!("{:?}", TNumber::Int(1) ^ TNumber::Int(0));
    println!("{:?}", TNumber::Int(1) | TNumber::Int(1));
    println!("{:?}",  !TNumber::Int(1).shl(10.into()));
    println!("{:?}", TNumber::Float(3.1) << TNumber::Float(1.0));
}
