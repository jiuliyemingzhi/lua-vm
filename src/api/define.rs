use std::rc::Rc;
use crate::err::err::{VmError, VmResult};
use crate::number::math::TNumber;

#[derive(Clone, Debug)]
pub enum LuaType {
    Nil,
    Boolean(bool),
    Number(TNumber),
    LightUserData(Rc<TLightUserData>),
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

    pub fn len(self) -> TNumber {
        match self {
            LuaType::LightUserData(v) => v.length(),
            LuaType::String(v) => (v.len() as i64).into(),
            LuaType::Table(v) => v.length(),
            LuaType::UserData(v) => v.length(),
            _ => panic!("attempt to get length of a xx value"),
        }
    }

    pub fn to_lua_str(&self) -> VmResult<Rc<String>> {
        match self {
            LuaType::Number(v) => Ok(Rc::new(v.to_string())),
            LuaType::String(v) => Ok(v.clone()),
            _ => Err(VmError::TypeChange(format!("{} 类型无法转换成string", self.get_typ_name()))),
        }
    }

    pub fn join_str(&self, mut lua_str: String) -> VmResult<String> {
        lua_str.push_str(self.to_lua_str()?.as_str());
        Ok(lua_str)
    }

    pub fn get_typ_name(&self) -> &str {
        match self {
            LuaType::Nil => "Nil",
            LuaType::Boolean(_) => "Boolean",
            LuaType::Number(_) => "Number",
            LuaType::LightUserData(_) => "LightUserData",
            LuaType::String(_) => "String",
            LuaType::Table(_) => "Table",
            LuaType::UserData(_) => "UserData",
            LuaType::Thread(_) => "Thread",
        }
    }

    pub fn to_number(&self) -> TNumber {
        self.to_number_x().unwrap_or(TNumber::default())
    }

    pub fn to_number_x(&self) -> VmResult<TNumber> {
        match self {
            LuaType::Number(v) => Ok(*v),
            LuaType::String(v) => Ok(v.as_str().into()),
            _ => Err(VmError::TypeChange(format!("{}转换类型失败!", self.get_typ_name()))),
        }
    }

    pub fn as_lua_str_x(&mut self) -> VmResult<Rc<String>> {
        match self {
            LuaType::Number(v) => {
                let new_str = Rc::new(v.to_string());
                *self = LuaType::String(new_str.clone());
                Ok(new_str)
            }
            _ => Err(VmError::TypeChange(format!("{}转换类型失败!", self.get_typ_name()))),
        }
    }

    pub fn string_or_set(&mut self) -> Rc<String> {
        self.as_lua_str_x().unwrap_or(Rc::new("".to_string()))
    }
}


impl From<String> for LuaType {
    fn from(value: String) -> Self {
        Self::String(Rc::new(value))
    }
}

impl From<i64> for LuaType {
    fn from(value: i64) -> Self {
        LuaType::Number(TNumber::Int(value))
    }
}

impl From<f64> for LuaType {
    fn from(value: f64) -> Self {
        LuaType::Number(TNumber::Float(value))
    }
}

impl From<bool> for LuaType {
    fn from(value: bool) -> Self {
        LuaType::Boolean(value)
    }
}

#[derive(Debug)]
pub struct TLightUserData {}

impl TLightUserData {
    fn length(&self) -> TNumber {
        todo!()
    }
}

#[derive(Debug)]
pub struct TTable {}

impl TTable {
    fn length(&self) -> TNumber {
        todo!()
    }
}

#[derive(Debug)]
pub struct TUserData {}

impl TUserData {
    fn length(&self) -> TNumber {
        todo!()
    }
}

#[derive(Debug)]
pub struct TThread {}
