pub enum VmError {
    TypeChange(String),
}

pub type VmResult<T> = Result<T, VmError>;