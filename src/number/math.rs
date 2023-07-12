use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitXor, BitOr, Add, Rem, Sub, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub enum NaN {
    StringConvertNumber,
    FloatBit,
}


#[derive(Debug, Copy, Clone)]
pub enum TNumber {
    Int(i64),
    Float(f64),
    NaN(NaN),
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
            _ => write!(f, "NaN"),
        }
    }
}

macro_rules! operator_overload {
    ($($operator_name:ident => $ff:expr; $ii:expr),*) => {
        $(
            pub fn $operator_name(self, rhs: Self) -> Self {
                match self {
                    TNumber::Float(v) => match rhs {
                        TNumber::Float(r) => $ff(v, r).into(),
                        TNumber::Int(r) => $ff(v, r as f64).into(),
                        _ => rhs,
                    }
                    TNumber::Int(v) => match rhs {
                        TNumber::Float(r) => $ff(v as f64, r).into(),
                        TNumber::Int(r) => $ii(v, r).into(),
                        _ => rhs,
                    }
                    _ => self,
                }
            }
        )*
    };
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

    operator_overload!(
        add => f64::add; i64::wrapping_add,
        sub => f64::sub; i64::wrapping_sub,
        mul => f64::mul; i64::wrapping_mul,
        div => f64::div; |v, r| v as f64 / r as f64,
        rem => f64::rem; i64::wrapping_rem,
        shl => |_, _| NaN::FloatBit; |v: i64, r: i64|  v.wrapping_shl(if r >= 0 { r as u32 } else { -r as u32 } ),
        shr => |_, _| NaN::FloatBit;  |v: i64, r: i64| v.wrapping_shr(if r >= 0 { r as u32 } else { -r as u32 } ),
        bitor => |_, _| NaN::FloatBit; i64::bitor,
        bitand => |_, _| NaN::FloatBit; i64::bitand,
        bitxor => |_, _| NaN::FloatBit; i64::bitxor
    );


    pub fn neg(self) -> Self {
        match self {
            TNumber::Float(v) => { (-v).into() }
            TNumber::Int(v) => { (-v).into() }
            _ => self
        }
    }

    pub fn not(self) -> Self {
        match self {
            TNumber::Float(_) => NaN::FloatBit.into(),
            TNumber::Int(v) => (!v).into(),
            _ => self,
        }
    }
}


macro_rules! eq_overwrite {
    ($($eq:path => $func:ident, $res:ty, $def:ident),*) => {
        impl Eq for TNumber {}
        $(
            impl $eq for TNumber {
                fn $func(&self, rhs: &Self) -> $res {
                    match *self {
                        TNumber::Float(v) => match *rhs {
                            TNumber::Float(r) => v.$func(&r).into(),
                            TNumber::Int(r) => v.$func(&(r as f64)),
                            _ => $def,
                        }
                        TNumber::Int(v) => match *rhs {
                            TNumber::Float(r) => (v as f64).$func(&r),
                            TNumber::Int(r) => v.$func(&r),
                            _ => $def,
                        }
                        _ => $def,
                    }
                }
            }
        )*
    };
}

eq_overwrite!(
    PartialEq<Self> => eq, bool, false,
    PartialOrd<Self> => partial_cmp, Option<std::cmp::Ordering>, None
);

impl From<NaN> for TNumber {
    fn from(value: NaN) -> Self {
        Self::NaN(value)
    }
}

impl From<&str> for TNumber {
    fn from(s: &str) -> Self {
        if let Ok(v) = s.parse() {
            TNumber::Int(v)
        } else if let Ok(v) = s.parse() {
            TNumber::Float(v)
        } else {
            TNumber::NaN(NaN::StringConvertNumber)
        }
    }
}



