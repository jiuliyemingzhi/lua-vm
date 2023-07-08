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
    ($count:expr, $($name:ident => ($flag:ident, $arg_a:ident, $arg_b:ident, $i:ident)),*) => {
        #[derive(Debug)]
        #[repr(u32)]
        pub enum OpCode {
            $(
            $name,
            )*
        }

        pub mod code {
            use crate::vm::opcodes::{OpArg, OpCode, Operation, OpI, Flags};

            $(
            pub const $name: Operation = Operation::new(Flags::$flag, OpArg::$arg_a, OpArg::$arg_b, OpI::$i, OpCode::$name);
            )*
            pub static OPERATION_ALL: [&Operation; $count] = [$(&$name,)*];
        }

        impl OpCode {
            pub fn get_option(self) -> &'static Operation {
                match self {
                    $(
                    OpCode::$name => &code::$name,
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

    pub fn is_test(&self) -> bool { self.flag.contains(Flags::TEST) }

    pub fn is_set_a(&self) -> bool { self.flag.contains(Flags::SET_A) }
}

impl From<OpCode> for u32 {
    fn from(value: OpCode) -> Self { value as u32 }
}

operation!(
    3,
    MOVE => (SET_A, R, N, ABC),
    LOADK => (SET_A, K, N, ABx),
    TESTTEST => (SET_A_TEST, R, U, ABC)
);


