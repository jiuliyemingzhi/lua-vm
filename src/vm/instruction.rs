use crate::vm::opcodes::{OpCode, Operation, code::OPERATION_ALL};

const OPERATION_LEN: u32 = 6;
const OPERATION_MAX: u32 = 1 << OPERATION_LEN;
const A_LEN: u32 = 8;
const A_MAX: u32 = 1 << A_LEN;
const B_C_LEN: u32 = 9;
const B_C_MAX: u32 = 1 << B_C_LEN;
const SBX_MAX: isize = ((1 << 18) - 1) >> 1;

impl From<u32> for &'static Operation {
    fn from(value: u32) -> Self { OPERATION_ALL[(value & OPERATION_MAX) as usize] }
}

macro_rules! u32opi {
    ($($name:ident => $($field:ident:$func:ident),*);*) => {
        $(
            pub struct $name{
                pub operation: &'static Operation,
                $(pub $field: usize,)*
            }

            impl $name {
                pub fn new(v: u32) -> Self { Self{ operation: v.into(), $($field: $func(v),)* } }

                pub fn reset(&mut self, v: u32) {
                    self.operation = v.into();
                    $(self.$field = $func(v);)*
                }
            }
        )*
    };
}

u32opi!(
    ABC => a:u32a, b:u32b, c:u32c;
    ABx => a:u32a, bx:u32bx;
    Ax  => ax:u32ax
);

impl ABx {
    pub fn get_sbx(&self) -> isize { self.bx as isize - SBX_MAX }
}

macro_rules! u32_to_usize {
    ($($name:ident => $body:expr),*) => {
        $(
           #[inline]
           pub fn $name(v: u32) -> usize { (v >> $body) as usize }
        )*
    };
}

u32_to_usize!(
    u32a => OPERATION_LEN & A_MAX,
    u32b => (OPERATION_LEN + A_LEN) & B_C_MAX,
    u32c => (OPERATION_LEN + A_LEN + B_C_LEN) & B_C_MAX,
    u32ax => OPERATION_LEN,
    u32bx => OPERATION_LEN + A_LEN
);

