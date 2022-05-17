use edu_asm_assembler::register::{G_0_CODE, G_1_CODE, G_3_CODE, G_2_CODE, G_4_CODE, G_5_CODE, G_6_CODE, G_7_CODE, S_B_CODE, S_E_CODE, R_CODE, I_CODE, Z_CODE, E_CODE};

use crate::state::VmState;

pub trait Writeable {
    fn set_signed(self, state: &mut VmState, value: i64);
    fn set_unsigned(self, state: &mut VmState, value: u64);
}

pub trait Readable {
    fn get_signed(self, state: &mut VmState) -> i64;
    fn get_unsigned(self, state: &mut VmState) -> u64;
}

pub trait WriteableReadable: Writeable + Readable {}

impl<T: Writeable + Readable> WriteableReadable for T {}

pub trait Pointerable: Readable {
    fn get_pointer(self, state: &mut VmState) -> usize;
}

impl<R: Readable> Pointerable for R {
    fn get_pointer(self, state: &mut VmState) -> usize {
        let read_value = self.get_unsigned(state);
        usize::try_from(read_value).expect("the host architecture is 32-bit and the value in the pointer is to big")
    }
}

pub enum RegImm {
    Immediate(u64),
    Register(RegisterSpec),
}

#[repr(u8)]
pub enum RegisterSpec {
    G0 = G_0_CODE,
    G1 = G_1_CODE,
    G2 = G_2_CODE,
    G3 = G_3_CODE,
    G4 = G_4_CODE,
    G5 = G_5_CODE,
    G6 = G_6_CODE,
    G7 = G_7_CODE,
    SB = S_B_CODE,
    SE = S_E_CODE,
    R = R_CODE,
    I = I_CODE,
    Z = Z_CODE,
    E = E_CODE,
}

impl Writeable for RegisterSpec {
    #[cfg_attr(not(wasm), inline(always))]
    fn set_signed(self, state: &mut VmState, value: i64) {
        match self {
            RegisterSpec::G0 => state.registers.g.0.set_signed(value),
            RegisterSpec::G1 => state.registers.g.1.set_signed(value),
            RegisterSpec::G2 => state.registers.g.2.set_signed(value),
            RegisterSpec::G3 => state.registers.g.3.set_signed(value),
            RegisterSpec::G4 => state.registers.g.4.set_signed(value),
            RegisterSpec::G5 => state.registers.g.5.set_signed(value),
            RegisterSpec::G6 => state.registers.g.6.set_signed(value),
            RegisterSpec::G7 => state.registers.g.7.set_signed(value),
            RegisterSpec::SB => state.registers.s.b.set_signed(value),
            RegisterSpec::SE => state.registers.s.e.set_signed(value),
            RegisterSpec::R => state.registers.m.r.set_signed(value),
            RegisterSpec::I => state.registers.m.i.set_signed(value),
            RegisterSpec::Z => {},
            RegisterSpec::E => state.registers.m.e.set_signed(value),
        }
    }

    #[cfg_attr(not(wasm), inline(always))]
    fn set_unsigned(self, state: &mut VmState, value: u64) {
        match self {
            RegisterSpec::G0 => state.registers.g.0.set_unsigned(value),
            RegisterSpec::G1 => state.registers.g.1.set_unsigned(value),
            RegisterSpec::G2 => state.registers.g.2.set_unsigned(value),
            RegisterSpec::G3 => state.registers.g.3.set_unsigned(value),
            RegisterSpec::G4 => state.registers.g.4.set_unsigned(value),
            RegisterSpec::G5 => state.registers.g.5.set_unsigned(value),
            RegisterSpec::G6 => state.registers.g.6.set_unsigned(value),
            RegisterSpec::G7 => state.registers.g.7.set_unsigned(value),
            RegisterSpec::SB => state.registers.s.b.set_unsigned(value),
            RegisterSpec::SE => state.registers.s.e.set_unsigned(value),
            RegisterSpec::R => state.registers.m.r.set_unsigned(value),
            RegisterSpec::I => state.registers.m.i.set_unsigned(value),
            RegisterSpec::Z => {},
            RegisterSpec::E => state.registers.m.e.set_unsigned(value),
        }
    }
}

impl Readable for RegisterSpec {
    fn get_signed(self, state: &mut VmState) -> i64 {
        const ZERO_VALUE: i64 = 0;
        match self {
            RegisterSpec::G0 => state.registers.g.0.get_signed(),
            RegisterSpec::G1 => state.registers.g.1.get_signed(),
            RegisterSpec::G2 => state.registers.g.2.get_signed(),
            RegisterSpec::G3 => state.registers.g.3.get_signed(),
            RegisterSpec::G4 => state.registers.g.4.get_signed(),
            RegisterSpec::G5 => state.registers.g.5.get_signed(),
            RegisterSpec::G6 => state.registers.g.6.get_signed(),
            RegisterSpec::G7 => state.registers.g.7.get_signed(),
            RegisterSpec::SB => state.registers.s.b.get_signed(),
            RegisterSpec::SE => state.registers.s.e.get_signed(),
            RegisterSpec::R => state.registers.m.r.get_signed(),
            RegisterSpec::I => state.registers.m.i.get_signed(),
            RegisterSpec::Z => ZERO_VALUE,
            RegisterSpec::E => state.registers.m.e.get_signed(),
        }
    }

    fn get_unsigned(self, state: &mut VmState) -> u64 {
        const ZERO_VALUE: u64 = 0;
        match self {
            RegisterSpec::G0 => state.registers.g.0.get_unsigned(),
            RegisterSpec::G1 => state.registers.g.1.get_unsigned(),
            RegisterSpec::G2 => state.registers.g.2.get_unsigned(),
            RegisterSpec::G3 => state.registers.g.3.get_unsigned(),
            RegisterSpec::G4 => state.registers.g.4.get_unsigned(),
            RegisterSpec::G5 => state.registers.g.5.get_unsigned(),
            RegisterSpec::G6 => state.registers.g.6.get_unsigned(),
            RegisterSpec::G7 => state.registers.g.7.get_unsigned(),
            RegisterSpec::SB => state.registers.s.b.get_unsigned(),
            RegisterSpec::SE => state.registers.s.e.get_unsigned(),
            RegisterSpec::R => state.registers.m.r.get_unsigned(),
            RegisterSpec::I => state.registers.m.i.get_unsigned(),
            RegisterSpec::Z => ZERO_VALUE,
            RegisterSpec::E => state.registers.m.e.get_unsigned(),
        }
    }
}
