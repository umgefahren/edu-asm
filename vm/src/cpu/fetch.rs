#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(serde, derivce(serde::Serialize, serde::Deserialize))]
pub struct ControlUnit {
    pub instruction_regiser: [u8; 3],
}


