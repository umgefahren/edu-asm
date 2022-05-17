//! Simulates the CPU in the von-neumann architecture

use self::fetch::ControlUnit;

pub mod fetch;
pub mod decode;
pub mod instruction;


pub struct CPU {
    pub control_unit: ControlUnit,
}
