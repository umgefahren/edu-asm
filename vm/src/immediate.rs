#[derive(Debug, Clone, Copy)]
pub enum RegisterImmediate {
    Register,
    Immediate,
    RegisterImmediate,
    None
}

pub struct Immediate(u64);
