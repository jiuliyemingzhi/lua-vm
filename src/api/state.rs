use crate::api::define::LuaType;

pub trait State {
    fn get_top(&self) -> usize;
    fn pop(&mut self, n: usize);
    fn copy(&mut self, from: usize, to: usize);
    fn push_value(&mut self, idx: usize);
    fn replace(&mut self, idx: usize);
    fn insert(&mut self, idx: usize);
    fn remove(&mut self, idx: usize);
    fn rotate(&mut self, idx: usize, n: usize);
    fn set_top(&mut self, idx: usize);
    fn push(&mut self, v: LuaType);
    fn get(&self, idx: usize) -> &LuaType;
}