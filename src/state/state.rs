use crate::api::define::{LuaType, OptionLuaType};
use crate::api::state::State;
use crate::state::stack::LuaStack;
use crate::vm::opcodes::Operation;

struct LuaState {
    stack: LuaStack,
}

impl LuaState {
    fn new() -> Self {
        Self {
            stack: LuaStack::new(1024)
        }
    }
}

impl State for LuaState {
    fn get_top(&self) -> usize {
        self.stack.get_top()
    }

    fn pop(&mut self, n: usize) {
        self.stack.discard(n);
    }

    fn copy(&mut self, from: usize, to: usize) {
        self.stack.copy(from, to);
    }

    fn push_value(&mut self, idx: usize) {
        self.stack.push(self.stack.get(idx));
    }

    fn replace(&mut self, idx: usize) {
        let pop = self.stack.pop();
        self.stack.set(idx, pop);
    }

    fn insert(&mut self, idx: usize) {
        self.stack.rotate(idx, 1);
    }

    fn remove(&mut self, idx: usize) {
        self.stack.remove(idx);
    }

    fn rotate(&mut self, idx: usize, n: usize) {
        self.stack.rotate(idx, n);
    }

    fn set_top(&mut self, idx: usize) {
        self.stack.set_top(idx);
    }

    fn push(&mut self, v: LuaType) {
        self.stack.push(Some(v));
    }

    fn get(&self, idx: usize) -> &OptionLuaType {
        self.stack.index(idx)
    }
}

