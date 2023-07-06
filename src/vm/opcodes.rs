use bitflags::bitflags;

#[derive(Debug)]
pub enum OpI {
    ABC,
    ABx,
    AsBx,
    Ax,
}

#[derive(Debug)]
pub enum OpArg {
    N,
    U,
    R,
    K,
}

bitflags! {
    #[derive(Debug)]
    pub struct Flags: u8 {
        const NONE = 0;
        const TEST = 1;
        const SET_A = 1 << 1;
        const SET_A_TEST = Self::TEST.bits() | Self::SET_A.bits();
    }
}

macro_rules! operation {
    ($($code:ident => ($flag:ident, $arg_a:ident, $arg_b:ident, $i:ident)),*) => {
#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    $(
    $code,
    )*
}
mod code {
    use crate::vm::opcodes::{OpArg, OpCode, Operation, OpI, Flags};

    $(
    pub const $code: Operation = Operation::new(Flags::$flag, OpArg::$arg_a, OpArg::$arg_b, OpI::$i, OpCode::$code);
    )*

}

impl OpCode {
    pub fn get_option(self) -> &'static Operation {
        match self {
            $(
            OpCode::$code => &code::$code,
            )*
        }
    }
}
    };
}

#[derive(Debug)]
pub struct Operation {
    flag: Flags,
    arg_a_mode: OpArg,
    arg_b_mode: OpArg,
    i: OpI,
    code: OpCode,
}

impl Operation {
    pub const fn new(flag: Flags, arg_a_mode: OpArg, arg_b_mode: OpArg, i: OpI, code: OpCode) -> Self {
        Self {
            flag,
            arg_a_mode,
            arg_b_mode,
            i,
            code,
        }
    }
}

operation!(
    MOVE => (SET_A, R, N, ABC),
    LOADK => (SET_A, K, N, ABx),
    TESTTEST => (SET_A_TEST, R, U, ABC),
    UNKOWN => (NONE, R, U, Ax)
);