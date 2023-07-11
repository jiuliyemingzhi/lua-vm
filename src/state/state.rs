use crate::api::define::LuaType;
use crate::state::stack::LuaStack;

#[derive(Debug)]
pub struct LuaState {
    stack: LuaStack,
}

impl LuaState {
    fn new() -> Self {
        Self {
            stack: LuaStack::new(1024)
        }
    }

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

    fn get_right_idx(&self, idx: usize) -> usize { self.get_top() - idx }

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
        self.stack.push(v);
    }

    fn get(&self, idx: usize) -> &LuaType {
        self.stack.index(idx)
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::api::define::{LuaType, TNumber};
    use crate::api::state::State;
    use crate::state::state::LuaState;

    #[test]
    fn test_all_state() {
        let mut state = LuaState::new();
        state.push(LuaType::Boolean(true));
        state.push(LuaType::Number(TNumber::Int(10)));
        state.push(LuaType::Nil);
        state.push(LuaType::String(Rc::new("hello".to_string())));
        println!("{:?}", state);
        state.push_value(state.get_right_idx(4));
        println!("{:?}", state);
        state.replace(2);
        println!("{:?}", state);
        state.set_top(6);
        println!("{:?}", state);
        state.remove(state.get_right_idx(3));
        println!("{:?}", state);
        state.set_top(state.get_right_idx(4));
        println!("{:?}", state);
    }
}