use std::ops::Deref;
use std::rc::Rc;
use crate::number::math::TNumber;

#[derive(Clone, Debug)]
pub enum LuaType {
    Nil,
    Boolean(bool),
    LightUserData(Rc<TLightUserData>),
    Number(TNumber),
    String(Rc<String>),
    Table(Rc<TTable>),
    UserData(Rc<TUserData>),
    Thread(Rc<TThread>),
}

impl LuaType {
    pub fn reset(&mut self, v: LuaType) { *self = v; }

    pub fn set_nil(&mut self) { *self = Self::Nil; }

    pub fn is_nil(&self) -> bool {
        match self {
            LuaType::Nil => true,
            _ => false
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            LuaType::Nil => false,
            LuaType::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn to_number(&self) -> TNumber {
        self.to_number_x().unwrap_or(TNumber::default_float())
    }

    pub fn to_number_x(&self) -> Option<TNumber> {
        match self {
            LuaType::Number(v) => Some(v.clone().to_float()),
            _ => None,
        }
    }

    pub fn to_integer(&self) -> TNumber {
        self.to_integer_x().unwrap_or(TNumber::default())
    }

    pub fn to_integer_x(&self) -> Option<TNumber> {
        match self {
            LuaType::Number(v) => Some(v.clone().to_integer()),
            _ => None,
        }
    }

    pub fn string_or_set_x(&mut self) -> Option<Rc<String>> {
        match self {
            LuaType::Number(v) => {
                let new_str = Rc::new(v.to_string());
                *self = LuaType::String(new_str.clone());
                Some(new_str)
            }
            _ => None,
        }
    }

    pub fn string_or_set(&mut self) -> Rc<String> {
        self.string_or_set_x().unwrap_or(Rc::new("".to_string()))
    }
}

#[derive(Debug)]
pub struct TLightUserData {}


#[derive(Debug)]
pub struct TTable {}

#[derive(Debug)]
pub struct TUserData {}

#[derive(Debug)]
pub struct TThread {}
