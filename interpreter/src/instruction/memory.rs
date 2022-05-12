use edu_asm_parser::instruction::Memory;

use crate::{
    behaviour::{Readable, Writeable},
    register::RegisterSpecifier,
};

use super::Executable;

pub(crate) struct Mov<T: Writeable, S: Readable> {
    t: T,
    s: S,
}

impl<T: Writeable, S: Readable> Executable for Mov<T, S> {
    fn execute(&self, state: &mut crate::State) {
        self.t.set_unsigned(state, self.s.get_unsigned(state))
    }
}

pub(crate) struct Push<D: Readable> {
    d: D,
}

impl<D: Readable> Executable for Push<D> {
    fn execute(&self, state: &mut crate::State) {
        let val = self.d.get_unsigned(state);
        state.stack.push(val);
    }
}

pub(crate) struct Pop<D: Writeable> {
    d: D,
}

impl<D: Writeable> Executable for Pop<D> {
    fn execute(&self, state: &mut crate::State) {
        let val = state.stack.pop().expect("stack is empty");
        self.d.set_unsigned(state, val);
    }
}

pub(super) fn transpile_memory(instr: Memory) -> Box<dyn Executable> {
    match instr {
        Memory::Mov { t, s } => {
            let t = RegisterSpecifier::from(t);
            let s = RegisterSpecifier::from(s);
            Box::new(Mov { t, s })
        }
        Memory::Push { d } => {
            let d = RegisterSpecifier::from(d);
            Box::new(Push { d })
        }
        Memory::Pop { d } => {
            let d = RegisterSpecifier::from(d);
            Box::new(Pop { d })
        }
    }
}
