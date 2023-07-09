use std::rc::Rc;

pub type OptionLuaType = Option<LuaType>;


#[derive(Clone)]
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

    pub fn is_nil(&self) -> bool { self == Self::Nil }

    pub fn to_bool(&self) -> bool {
        match self {
            LuaType::Nil => false,
            LuaType::Boolean(b) => b,
            _ => true,
        }
    }

    pub fn to_number(&self) -> f64 {
        self.to_number_x().unwrap_or(0f64)
    }

    pub fn to_number_x(&self) -> Option<f64> {
        match self {
            LuaType::Number(v) => Some(v.to_number()),
            _ => None,
        }
    }

    pub fn to_integer(&self) -> i64 {
        self.to_integer_x().unwrap_or(0)
    }

    pub fn to_integer_x(&self) -> Option<i64> {
        match self {
            LuaType::Number(v) => Some(v.to_integer()),
            _ => None,
        }
    }

    pub fn to_string_x(&self) -> Option<String> {
        match self {
            LuaType::Number(v) => Some(v.to_string()),
            _ => None,
        }
    }
}

pub struct TLightUserData {}

#[derive(Clone)]
pub enum TNumber {
    Float(f64),
    Int(i64),
}

impl TNumber {
    pub fn to_string(&self) -> String {
        match self {
            TNumber::Float(v) => { v.to_string() }
            TNumber::Int(v) => { v.to_string() }
        }
    }

    pub fn to_number(&self) -> f64 {
        match self {
            TNumber::Float(&n) => n,
            LTNumber::Int(&n) => n as f64,
        }
    }

    pub fn to_integer(&self) -> i64 {
        match self {
            TNumber::Int(&n) => n,
            LTNumber::Float(&n) => n as i64,
        }
    }
}

pub struct TTable {}

pub struct TUserData {}

pub struct TThread {}
