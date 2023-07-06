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

macro_rules! operation {
    ($($code:ident => ($flag:expr, $arg_a:ident, $arg_b:ident, $i:ident)),*) => {
#[derive(Debug)]
pub enum OpCode {
    $(
    $code,
    )*
}
mod code {
    use crate::vm::opcodes::{OpArg, OpCode, Operation, OpI};

    pub const TEST_FLAG: u8 = 1;
    pub const SET_A_FLAG: u8 = 1 << 1;

    $(
    pub const $code: Operation = Operation::new($flag, OpArg::$arg_a, OpArg::$arg_b, OpI::$i, OpCode::$code);
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

operation!(
    MOVE => (SET_A_FLAG, R, N, ABC),
    LOADK => (SET_A_FLAG, K, N, ABx),
    TESTTEST => (SET_A_FLAG | TEST_FLAG, R, U, ABC)
);

#[derive(Debug)]
pub struct Operation {
    flag: u8,
    arg_a_mode: OpArg,
    arg_b_mode: OpArg,
    i: OpI,
    code: OpCode,
}

impl Operation {
    pub const fn new(flag: u8, arg_a_mode: OpArg, arg_b_mode: OpArg, i: OpI, code: OpCode) -> Self {
        Self {
            flag,
            arg_a_mode,
            arg_b_mode,
            i,
            code,
        }
    }
}