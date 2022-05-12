use std::fmt::Debug;

use edu_asm_parser::register::RegisterToken;

use crate::behaviour::{Readable, Writeable};

pub(crate) trait RegisterBehaviour {
    fn get_signed(&self) -> i64;
    fn get_unsigned(&self) -> u64;
    fn set_signed(&mut self, val: i64);
    fn set_unsigned(&mut self, val: u64);
}

#[derive(Default)]
pub(crate) struct RegisterCollection {
    gp: GeneralPurposeRegisters,
    pub(crate) s: StackRegisters,
    pub(crate) m: MiscRegisters,
}

#[derive(Default)]
pub(crate) struct GeneralPurposeRegisters {
    g0: Register,
    g1: Register,
    g2: Register,
    g3: Register,
    g4: Register,
    g5: Register,
    g6: Register,
    g7: Register,
}

#[derive(Default)]
pub(crate) struct StackRegisters {
    beg: ZeroRegister,
    end: ZeroRegister,
}

#[derive(Default)]
pub(crate) struct MiscRegisters {
    pub(crate) ret: Register,
    pub(crate) ins: InstructionRegister,
    pub(crate) zer: ZeroRegister,
}

#[derive(Default)]
pub(crate) struct ZeroRegister {}

impl RegisterBehaviour for ZeroRegister {
    #[inline]
    fn get_signed(&self) -> i64 {
        0b0
    }

    #[inline]
    fn get_unsigned(&self) -> u64 {
        0b0
    }

    #[inline]
    fn set_signed(&mut self, _: i64) {}

    #[inline]
    fn set_unsigned(&mut self, _: u64) {}
}

#[derive(Copy, Clone)]
pub(crate) union Register {
    signed: i64,
    unsigned: u64,
}

impl Default for Register {
    fn default() -> Self {
        Register { signed: 0 }
    }
}

impl Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary_str: String = format!("{:#b}", unsafe { self.unsigned });
        f.debug_struct("Register")
            .field("binary", &binary_str)
            .field("signed", &unsafe { self.signed })
            .field("unsigned", &unsafe { self.unsigned })
            .finish()
    }
}

impl RegisterBehaviour for Register {
    #[inline]
    fn get_signed(&self) -> i64 {
        unsafe { self.signed }
    }

    #[inline]
    fn get_unsigned(&self) -> u64 {
        unsafe { self.unsigned }
    }

    #[inline]
    fn set_signed(&mut self, val: i64) {
        self.signed = val;
    }

    #[inline]
    fn set_unsigned(&mut self, val: u64) {
        self.unsigned = val;
    }
}

#[derive(Default)]
pub(crate) struct InstructionRegister {
    counter: usize,
}

impl InstructionRegister {
    pub(crate) fn inc(&mut self) -> usize {
        let ret = self.counter;
        self.counter += 1;
        ret
    }

    pub(crate) fn jump(&mut self, loc: usize) {
        self.counter = loc;
    }
}

impl RegisterBehaviour for InstructionRegister {
    #[inline]
    fn get_signed(&self) -> i64 {
        let tmp = self.counter as u64;
        tmp as i64
    }

    #[inline]
    fn get_unsigned(&self) -> u64 {
        self.counter as u64
    }

    #[cold]
    fn set_signed(&mut self, val: i64) {
        let tmp: u64 = unsafe { std::mem::transmute(val) };
        self.counter = usize::try_from(tmp)
            .expect("value written into the program counter exceeded the archtiectures limits");
    }

    #[cold]
    fn set_unsigned(&mut self, val: u64) {
        self.counter = usize::try_from(val)
            .expect("value written into the program counter exceeded the archtiectures limits")
    }
}

pub(crate) enum RegisterSpecifier {
    G0,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    SB,
    SE,
    R,
    I,
    Z,
}

impl Readable for RegisterSpecifier {
    #[inline]
    fn get_signed(&self, state: &crate::State) -> i64 {
        match self {
            RegisterSpecifier::G0 => state.registers.gp.g0.get_signed(),
            RegisterSpecifier::G1 => state.registers.gp.g1.get_signed(),
            RegisterSpecifier::G2 => state.registers.gp.g2.get_signed(),
            RegisterSpecifier::G3 => state.registers.gp.g3.get_signed(),
            RegisterSpecifier::G4 => state.registers.gp.g4.get_signed(),
            RegisterSpecifier::G5 => state.registers.gp.g5.get_signed(),
            RegisterSpecifier::G6 => state.registers.gp.g6.get_signed(),
            RegisterSpecifier::G7 => state.registers.gp.g7.get_signed(),
            RegisterSpecifier::SB => state.registers.s.beg.get_signed(),
            RegisterSpecifier::SE => state.registers.s.end.get_signed(),
            RegisterSpecifier::R => state.registers.m.ret.get_signed(),
            RegisterSpecifier::I => state.registers.m.ins.get_signed(),
            RegisterSpecifier::Z => state.registers.m.zer.get_signed(),
        }
    }

    #[inline]
    fn get_unsigned(&self, state: &crate::State) -> u64 {
        match self {
            RegisterSpecifier::G0 => state.registers.gp.g0.get_unsigned(),
            RegisterSpecifier::G1 => state.registers.gp.g1.get_unsigned(),
            RegisterSpecifier::G2 => state.registers.gp.g2.get_unsigned(),
            RegisterSpecifier::G3 => state.registers.gp.g3.get_unsigned(),
            RegisterSpecifier::G4 => state.registers.gp.g4.get_unsigned(),
            RegisterSpecifier::G5 => state.registers.gp.g5.get_unsigned(),
            RegisterSpecifier::G6 => state.registers.gp.g6.get_unsigned(),
            RegisterSpecifier::G7 => state.registers.gp.g7.get_unsigned(),
            RegisterSpecifier::SB => state.registers.s.beg.get_unsigned(),
            RegisterSpecifier::SE => state.registers.s.end.get_unsigned(),
            RegisterSpecifier::R => state.registers.m.ret.get_unsigned(),
            RegisterSpecifier::I => state.registers.m.ins.get_unsigned(),
            RegisterSpecifier::Z => state.registers.m.zer.get_unsigned(),
        }
    }
}

impl Writeable for RegisterSpecifier {
    #[inline]
    fn set_signed(&self, state: &mut crate::State, val: i64) {
        match self {
            RegisterSpecifier::G0 => state.registers.gp.g0.set_signed(val),
            RegisterSpecifier::G1 => state.registers.gp.g1.set_signed(val),
            RegisterSpecifier::G2 => state.registers.gp.g2.set_signed(val),
            RegisterSpecifier::G3 => state.registers.gp.g3.set_signed(val),
            RegisterSpecifier::G4 => state.registers.gp.g4.set_signed(val),
            RegisterSpecifier::G5 => state.registers.gp.g5.set_signed(val),
            RegisterSpecifier::G6 => state.registers.gp.g6.set_signed(val),
            RegisterSpecifier::G7 => state.registers.gp.g7.set_signed(val),
            RegisterSpecifier::SB => state.registers.s.beg.set_signed(val),
            RegisterSpecifier::SE => state.registers.s.end.set_signed(val),
            RegisterSpecifier::R => state.registers.m.ret.set_signed(val),
            RegisterSpecifier::I => state.registers.m.ins.set_signed(val),
            RegisterSpecifier::Z => state.registers.m.zer.set_signed(val),
        }
    }

    #[inline]
    fn set_unsigned(&self, state: &mut crate::State, val: u64) {
        match self {
            RegisterSpecifier::G0 => state.registers.gp.g0.set_unsigned(val),
            RegisterSpecifier::G1 => state.registers.gp.g1.set_unsigned(val),
            RegisterSpecifier::G2 => state.registers.gp.g2.set_unsigned(val),
            RegisterSpecifier::G3 => state.registers.gp.g3.set_unsigned(val),
            RegisterSpecifier::G4 => state.registers.gp.g4.set_unsigned(val),
            RegisterSpecifier::G5 => state.registers.gp.g5.set_unsigned(val),
            RegisterSpecifier::G6 => state.registers.gp.g6.set_unsigned(val),
            RegisterSpecifier::G7 => state.registers.gp.g7.set_unsigned(val),
            RegisterSpecifier::SB => state.registers.s.beg.set_unsigned(val),
            RegisterSpecifier::SE => state.registers.s.end.set_unsigned(val),
            RegisterSpecifier::R => state.registers.m.ret.set_unsigned(val),
            RegisterSpecifier::I => state.registers.m.ins.set_unsigned(val),
            RegisterSpecifier::Z => state.registers.m.zer.set_unsigned(val),
        }
    }
}

impl From<RegisterToken> for RegisterSpecifier {
    fn from(rt: RegisterToken) -> Self {
        match rt {
            RegisterToken::GeneralPurpose(0) => Self::G0,
            RegisterToken::GeneralPurpose(1) => Self::G1,
            RegisterToken::GeneralPurpose(2) => Self::G2,
            RegisterToken::GeneralPurpose(3) => Self::G3,
            RegisterToken::GeneralPurpose(4) => Self::G4,
            RegisterToken::GeneralPurpose(5) => Self::G5,
            RegisterToken::GeneralPurpose(6) => Self::G6,
            RegisterToken::GeneralPurpose(7) => Self::G7,
            RegisterToken::GeneralPurpose(_) => panic!("general purpose register index is invalid"),
            RegisterToken::StackBase => Self::SE,
            RegisterToken::StackEnd => Self::SB,
            RegisterToken::Return => Self::R,
            RegisterToken::Instruction => Self::I,
            RegisterToken::Zero => Self::Z,
        }
    }
}
