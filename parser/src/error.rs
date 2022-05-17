use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u64)]
pub enum Errors {
    Unknown = 1,
    BadInstruction = 2,
    Overflow = 3,
    DivZero = 4,
}

impl Errors {
    pub fn to_int(&self) -> u64 {
        *self as u64
    }
}

pub const UNKNOWN_ERROR_STR: &str = "eUnknown";
pub const BAD_INSTRUCTION_STR: &str = "eBadInstruction";
pub const OVERFLOW_STR: &str = "eOverflow";
pub const DIV_ZERO_STR: &str = "eDivZero";

impl FromStr for Errors {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            UNKNOWN_ERROR_STR => Ok(Errors::Unknown),
            BAD_INSTRUCTION_STR => Ok(Errors::BadInstruction),
            OVERFLOW_STR => Ok(Errors::Overflow),
            DIV_ZERO_STR => Ok(Errors::DivZero),
            _ => Err(()),
        }
    }
}
