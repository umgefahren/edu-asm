use std::{intrinsics::transmute, ops::{Add, AddAssign, Sub, SubAssign}};

use edu_asm_parser::error::Errors;
use rand::{thread_rng, Rng};

use crate::settings::VmSettings;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
pub struct Register {
    value: u64,
}

impl Register {
    #[inline]
    pub const fn init_const(settings: &VmSettings) -> Self {
        assert!(settings.zero_initialized_register);
        Self {
            value: 0
        }
    }

    pub fn init(settings: &VmSettings) -> Self {
        if settings.zero_initialized_register {
            return Self::init_const(settings)
        }
        let mut rng = thread_rng();
        let value: u64 = rng.gen();
        Self {
            value
        }
    } 

    fn init_value(value: u64) -> Self {
        Self {
            value
        }
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) const fn get_signed(&self) -> i64 {
        unsafe { transmute(self.value) }
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) const fn get_unsigned(&self) -> u64 {
        self.value
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn set_signed(&mut self, value: i64) {
        self.value = unsafe { transmute(value) }
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn set_unsigned(&mut self, value: u64) {
        self.value = value
    }

}

impl Add for Register {
    type Output = Register;
    fn add(self, rhs: Self) -> Self::Output {
        Register {
            value: self.value + rhs.value
        }
    }
}

impl AddAssign for Register {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Register {
    type Output = Register;
    fn sub(self, rhs: Self) -> Self::Output {
       Register {
           value: self.value - rhs.value
       }
    }
}

impl SubAssign for Register {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
pub struct ProgramCounter {
    inner: Register,
}

impl ProgramCounter {
    pub const fn init_const(settings: &VmSettings) -> Self {
        Self {
            inner: Register::init_const(settings)
        }
    }

    pub fn init(settings: &VmSettings) -> Self {
        Self {
            inner: Register::init(settings)
        }
    }

    pub fn init_value(value: u64) -> Self {
        Self {
            inner: Register::init_value(value)
        }
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) const fn get_signed(&self) -> i64 {
        self.inner.get_signed()
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) const fn get_unsigned(&self) -> u64 {
        self.inner.get_unsigned()
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn set_signed(&mut self, value: i64) {
        self.inner.set_signed(value)
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn set_unsigned(&mut self, value: u64) {
        self.inner.set_unsigned(value)
    }

    pub(crate) fn pc(&self) -> usize {
        usize::try_from(self.inner.get_unsigned()).expect("the architecture the vm is running on doesn't support such a big program counter")
    }

    pub(crate) fn inc_pc(&mut self) -> usize {
        let ret = self.pc();
        let unsigned = self.get_unsigned();
        self.set_unsigned(unsigned + 1);
        ret
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
pub struct ZeroRegister {
    value: u64
}

impl ZeroRegister {
    pub const fn init() -> Self {
        Self {
            value: 0
        }
    }
}

impl Default for ZeroRegister {
    fn default() -> Self {
        Self::init()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorRegister {
    inner: Register,
}

impl ErrorRegister {
    pub const fn init_const(settings: &VmSettings) -> Self {
        Self {
            inner: Register::init_const(settings)
        }
    }

    pub fn init(settings: &VmSettings) -> Self {
        Self {
            inner: Register::init(settings)
        }
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn get_signed(&self) -> i64 {
        self.inner.get_signed()
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn get_unsigned(&self) -> u64 {
        self.inner.get_unsigned()
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn set_signed(&mut self, value: i64) {
        self.inner.set_signed(value)
    }

    #[cfg_attr(not(wasm), inline(always))]
    pub(crate) fn set_unsigned(&mut self, value: u64) {
        self.inner.set_unsigned(value)
    }

    pub(crate) fn clear(&mut self) {
        self.inner.set_unsigned(0)
    }

    pub(crate) fn set_error(&mut self, error: Errors) {
        self.inner.set_unsigned(error.to_int())
    }
} 
    


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
pub struct GeneralPurposeRegisters(pub Register, pub Register, pub Register, pub Register, pub Register, pub Register, pub Register, pub Register);

impl GeneralPurposeRegisters {
    pub const fn init_const(settings: &VmSettings) -> Self {
        Self {
            0: Register::init_const(settings),
            1: Register::init_const(settings),
            2: Register::init_const(settings),
            3: Register::init_const(settings),
            4: Register::init_const(settings),
            5: Register::init_const(settings),
            6: Register::init_const(settings),
            7: Register::init_const(settings),
        }
    }

    pub fn init(settings: &VmSettings) -> Self {
        Self {
            0: Register::init(settings),
            1: Register::init(settings),
            2: Register::init(settings),
            3: Register::init(settings),
            4: Register::init(settings),
            5: Register::init(settings),
            6: Register::init(settings),
            7: Register::init(settings),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
pub struct StackRegisters {
    /// the stack base pointer
    pub b: Register,
    /// the stack end pointer
    pub e: Register,
}

impl StackRegisters {
    pub const fn init_const(settings: &VmSettings) -> Self {
        Self {
            b: Register::init_const(settings),
            e: Register::init_const(settings),
        }
    }

    pub fn init(settings: &VmSettings) -> Self {
        Self {
            b: Register::init(settings),
            e: Register::init(settings),
        }
    }

    pub fn extend(&mut self) {
        self.e -= Register { value: 8 };
    }

    pub fn reduce(&mut self) {
        if self.e.value != self.b.value {
            self.e += Register { value: 8 };
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
pub struct MiscRegisters {
    /// the program counter
    pub i: ProgramCounter,
    /// the zero register
    pub z: ZeroRegister,
    /// the return register
    pub r: Register,
    /// the error register
    pub e: ErrorRegister,
}

impl MiscRegisters {
    pub const fn init_const(settings: &VmSettings) -> Self {
        Self {
            i: ProgramCounter::init_const(settings),
            z: ZeroRegister::init(),
            r: Register::init_const(settings),
            e: ErrorRegister::init_const(settings),
        }
    }

    pub fn init(settings: &VmSettings) -> Self {
        Self {
            i: ProgramCounter::init(settings),
            z: ZeroRegister::init(),
            r: Register::init(settings),
            e: ErrorRegister::init(settings),
        }
    }
}

pub struct RegisterCollection {
    pub g: GeneralPurposeRegisters,
    pub s: StackRegisters,
    pub m: MiscRegisters,
}
