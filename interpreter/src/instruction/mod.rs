use edu_asm_parser::instruction::{RegisterOrLiteral, Instruction};

use crate::{State, register::RegisterSpecifier, literal::Literal, behaviour::Readable};

pub(crate) mod arithmetic_base;
pub(crate) mod arithmetic_mult_div;
pub(crate) mod control_flow;
pub(crate) mod memory;
pub(crate) mod misc;

pub(crate) trait Executable {
    fn execute(&self, state: &mut State);
}

pub(crate) enum RegOrLit {
    Register(RegisterSpecifier),
    Literal(Literal),
}

impl From<RegisterOrLiteral> for RegOrLit {
    #[inline]
    fn from(rl: RegisterOrLiteral) -> Self {
        match rl {
            RegisterOrLiteral::Register(r) => RegOrLit::Register(r.into()),
            RegisterOrLiteral::Literal(l) => RegOrLit::Literal(l.into()),
        }
    }
}

impl Readable for RegOrLit {
    #[inline]
    fn get_signed(&self, state: &State) -> i64 {
        match self {
            RegOrLit::Register(r) => r.get_signed(state),
            RegOrLit::Literal(l)  => l.get_signed(state)
        }
    }

    #[inline]
    fn get_unsigned(&self, state: &State) -> u64 {
        match self {
            RegOrLit::Register(r) => r.get_unsigned(state),
            RegOrLit::Literal(l)  => l.get_unsigned(state)
        }
    }
}

#[inline]
pub(crate) fn transpile_instr(instr: Instruction) -> Box<dyn Executable> {
    match instr {
        Instruction::ArithmeticBase(i) => arithmetic_base::transpile_arithmetic_base(i),
        Instruction::ArithmeticMultDivEasy(i) => arithmetic_mult_div::transpile_arithmetic_mult_div(i),
        Instruction::ControlFlow(i) => control_flow::transpile_control_flow(i),
        Instruction::Memory(i) => memory::transpile_memory(i),
        Instruction::Misc(i) => misc::transpile_misc(i),
    }
}
