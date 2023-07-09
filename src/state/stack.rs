use std::ops::Index;
use crate::api::define::LuaType;
use crate::stack_overflow;
use crate::utils::lang::STACK_OVERFLOW;


#[derive(Debug)]
pub(crate) struct LuaStack {
    slots: Vec<LuaType>,
}


impl LuaStack {
    pub fn new(size: usize) -> Self {
        Self {
            slots: Vec::with_capacity(size),
        }
    }

    pub fn get_top(&self) -> usize { self.slots.len() }

    pub fn shrink_fit(&mut self) {
        if self.get_top() < self.slots.capacity() / 4 {
            self.slots.shrink_to(self.slots.capacity() / 2);
        }
    }

    pub fn push(&mut self, val: LuaType) {
        self.slots.push(val);
    }

    pub fn pop(&mut self) -> LuaType {
        self.slots.pop().expect(STACK_OVERFLOW)
    }

    pub fn discard(&mut self, n: usize) {
        self.slots.truncate(self.get_top() - n);
    }

    #[inline]
    pub fn not_valid(&self, idx: usize) -> bool {
        idx > self.get_top()
    }

    pub fn set(&mut self, idx: usize, val: LuaType) {
        if self.not_valid(idx) { stack_overflow!(); }
        self.slots[idx] = val;
    }

    #[inline]
    pub fn get(&self, idx: usize) -> LuaType {
        self.slots[idx].clone()
    }

    pub fn copy(&mut self, from: usize, to: usize) {
        self.slots[to] = self.slots[from].clone();
    }

    pub fn rotate(&mut self, idx: usize, n: usize) {
        self.slots[idx..].rotate_left(n);
    }

    pub fn remove(&mut self, idx: usize) {
        self.slots.remove(idx);
    }

    pub fn index(&self, idx: usize) -> &LuaType {
        self.slots.index(idx)
    }

    pub fn set_top(&mut self, idx: usize) {
        if self.get_top() != idx {
            self.slots.resize_with(idx, || LuaType::Nil);
        }
    }
}


