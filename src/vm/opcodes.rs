#[derive(Debug)]
pub enum OpI {
    ABC,
    AB,
    A,
    ABx,
    AsBx,
    Ax,
}

macro_rules! operation {
    ($count:expr, $($name:ident => ($i:ident)),*) => {
        #[derive(Debug)]
        #[repr(u32)]
        pub enum OpCode {
            $(
            $name,
            )*
        }

        pub mod code {
            use crate::vm::opcodes::{OpCode, Operation, OpI};

            $(
            pub const $name: Operation = Operation::new(OpI::$i, OpCode::$name);
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
    pub i: OpI,
    pub code: OpCode,
}

impl Operation {
    pub const fn new(i: OpI, code: OpCode) -> Self {
        Self {
            i,
            code,
        }
    }
}

impl From<OpCode> for u32 {
    fn from(value: OpCode) -> Self { value as u32 }
}

operation!(
    3,
    MOVE => (AB),
    LOADK => (ABx),
    TESTTEST => (ABC)
);
