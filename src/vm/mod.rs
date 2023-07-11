use crate::state::state::LuaState;

pub mod opcodes;
pub mod instruction;
pub mod handler;


pub struct Root {
    state: LuaState,
}

impl Root {
    pub fn start() {
        loop {}
    }
}