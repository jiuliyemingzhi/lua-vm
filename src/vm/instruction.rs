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

pub struct Instruction {
    operation: &'static Operation,
    v: u32,
}

macro_rules! u32_to_usize {
    ($($name:ident => $body:expr),*) => {
        $(
           #[inline]
           pub fn $name(&self) -> usize { (self.v >> $body) as usize }
        )*
    };
}

impl Instruction {
    #[inline]
    fn new(v: u32) -> Self {
        Self {
            v,
            operation: v.into(),
        }
    }

    #[inline]
    fn reset(&mut self, v: u32) {
        self.v = v;
        self.operation = v.into();
    }

    #[inline]
    pub fn get_sbx(&self) -> isize { self.get_bx() as isize - SBX_MAX }

    u32_to_usize!(
        get_a => OPERATION_LEN & A_MAX,
        get_b => (OPERATION_LEN + A_LEN) & B_C_MAX,
        get_c => (OPERATION_LEN + A_LEN + B_C_LEN) & B_C_MAX,
        get_ax => OPERATION_LEN,
        get_bx => OPERATION_LEN + A_LEN
    );
}






