use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::api::define::VmError;

#[derive(Debug, Copy, Clone)]
pub enum TNumber {
    Float(f64),
    Int(i64),
}

impl From<i64> for TNumber {
    fn from(value: i64) -> Self {
        TNumber::Int(value)
    }
}

impl From<f64> for TNumber {
    fn from(value: f64) -> Self {
        TNumber::Float(value)
    }
}

impl Display for TNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TNumber::Float(v) => write!(f, "{}", v),
            TNumber::Int(v) => write!(f, "{}", v),
        }
    }
}

impl TNumber {
    pub fn to_float(self) -> Self {
        match self {
            TNumber::Int(n) => TNumber::Float(n as f64),
            _ => self,
        }
    }

    pub fn to_integer(self) -> Self {
        match self {
            TNumber::Float(n) => TNumber::Int(n as i64),
            _ => self,
        }
    }

    pub fn default() -> Self { TNumber::Int(0) }


    pub fn default_float() -> Self { TNumber::Float(0f64) }

    pub fn floor_float(self) -> Self {
        match self {
            TNumber::Float(v) => v.floor().into(),
            _ => self.to_float(),
        }
    }
}

macro_rules! operator_overloading {
    ($($operator_name:ident => $operator_fn:ident; $ff:expr; $ii:expr),*) => {
        $(
            impl std::ops::$operator_name for TNumber {
                type Output = TNumber;

                fn $operator_fn(self, rhs: Self) -> Self::Output {
                    match self {
                        TNumber::Float(v) => {
                            match rhs {
                                TNumber::Float(r) => TNumber::Float($ff(v, r)),
                                TNumber::Int(r) => TNumber::Float($ff(v, r as f64)),
                            }
                        }
                        TNumber::Int(v) => {
                            match rhs {
                                TNumber::Float(r) => TNumber::Float($ff(v as f64, r)),
                                TNumber::Int(r) => $ii(v, r).into(),
                            }
                        }
                    }
                }
            }
        )*
    };
}


macro_rules! bit_not_float {
    () => {panic!("位运算不能有浮点数")};
}

operator_overloading!(
    Add => add; |v, r| v + r; |v:i64, r:i64| v.wrapping_add(r),
    Sub => sub; |v, r| v - r; |v:i64, r:i64| v.wrapping_sub(r),
    Mul => mul; |v, r| v * r; |v:i64, r:i64| v.wrapping_mul(r),
    Div => div; |v, r| v / r; |v:i64, r:i64| v as f64 / r as f64,
    Rem => rem; |v, r| v % r; |v:i64, r:i64| v.wrapping_rem(r),
    Shl => shl; |_, _| bit_not_float!(); |v:i64, r:i64| {
        if r >= 0 { v.overflowing_shl(r as u32).0 } else { v.overflowing_shr(-r as u32).0 }
    },
    Shr => shr; |_, _| bit_not_float!(); |v:i64, r:i64| {
        if r >= 0 { v.overflowing_shr(r as u32).0 } else { v.overflowing_shl(-r as u32).0 }
    },
    BitOr => bitor; |_, _| bit_not_float!(); |v:i64, r:i64| v.bitor(r),
    BitAnd => bitand; |_, _| bit_not_float!(); |v:i64, r:i64| v.bitand(r),
    BitXor => bitxor; |_, _| bit_not_float!(); |v:i64, r:i64| v.bitxor(r)
);

impl std::ops::Neg for TNumber {
    type Output = TNumber;

    fn neg(self) -> Self::Output {
        match self {
            TNumber::Float(v) => { (-v).into() }
            TNumber::Int(v) => { (-v).into() }
        }
    }
}

impl std::ops::Not for TNumber {
    type Output = TNumber;

    fn not(self) -> Self::Output {
        match self {
            TNumber::Float(_) => bit_not_float!(),
            TNumber::Int(v) => (!v).into(),
        }
    }
}

macro_rules! eq_overwrite {
    ($($eq:path => $func:ident, $res:ty),*) => {
        impl Eq for TNumber {}
        $(
            impl $eq for TNumber {
                fn $func(&self, rhs: &Self) -> $res {
                    match *self {
                        TNumber::Float(v) => {
                            match *rhs {
                                TNumber::Float(r) => v.$func(&r),
                                TNumber::Int(r) => v.$func(&(r as f64)),
                            }
                        }
                        TNumber::Int(v) => {
                            match *rhs {
                                TNumber::Float(r) => (v as f64).$func(&r),
                                TNumber::Int(r) => v.$func(&r),
                            }
                        }
                    }
                }
            }
        )*
    };
}

eq_overwrite!(
    PartialEq<Self> => eq, bool,
    PartialOrd<Self> => partial_cmp, Option<std::cmp::Ordering>
);

impl FromStr for TNumber {
    type Err = VmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse() {
            Ok(TNumber::Int(v))
        } else if let Ok(v) = s.parse() {
            Ok(TNumber::Float(v))
        } else {
            Err(VmError::StringConvertNumber)
        }
    }
}



