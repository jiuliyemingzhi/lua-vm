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

    pub fn string_or_set_x(&mut self) -> Option<String> {
        match self {
            LuaType::Number(v) => {
                let new_str = v.to_string();
                *self = LuaType::String(Rc::new(v.to_string()));
                Some(new_str)
            }
            _ => None,
        }
    }

    pub fn string_or_set(&mut self) -> String {
        self.string_or_set_x().unwrap_or("".to_string())
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
            TNumber::Float(v) => v.to_string(),
            TNumber::Int(v) => v.to_string()
        }
    }

    pub fn to_number(&self) -> f64 {
        match *self {
            TNumber::Float(n) => n,
            TNumber::Int(n) => n as f64,
        }
    }

    pub fn to_integer(&self) -> i64 {
        match *self {
            TNumber::Int(n) => n,
            TNumber::Float(n) => n as i64,
        }
    }
}

pub struct TTable {}

pub struct TUserData {}

pub struct TThread {}
