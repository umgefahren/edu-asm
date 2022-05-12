use edu_asm_parser::instruction::ControlFlow;

use crate::{
    behaviour::Readable,
    register::{RegisterBehaviour, RegisterSpecifier},
};

use super::Executable;

pub(crate) struct Jmp {
    loc: usize,
}

impl Executable for Jmp {
    fn execute(&self, state: &mut crate::State) {
        state.registers.m.ins.jump(self.loc);
    }
}

pub(crate) struct JmpEq<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpEq<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_unsigned(state);
        let r_val = self.r.get_unsigned(state);
        if l_val == r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpNe<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpNe<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_unsigned(state);
        let r_val = self.r.get_unsigned(state);
        if l_val != r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpGtS<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpGtS<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_unsigned(state);
        let r_val = self.r.get_unsigned(state);
        if l_val > r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpGeS<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpGeS<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_unsigned(state);
        let r_val = self.r.get_unsigned(state);
        if l_val >= r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpLtS<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpLtS<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_unsigned(state);
        let r_val = self.r.get_unsigned(state);
        if l_val < r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpLeS<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpLeS<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_unsigned(state);
        let r_val = self.r.get_unsigned(state);
        if l_val <= r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpGtU<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpGtU<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_signed(state);
        let r_val = self.r.get_signed(state);
        if l_val > r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpGeU<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpGeU<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_signed(state);
        let r_val = self.r.get_signed(state);
        if l_val >= r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpLtU<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpLtU<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_signed(state);
        let r_val = self.r.get_signed(state);
        if l_val < r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct JmpLeU<L: Readable, R: Readable> {
    l: L,
    r: R,
    loc: usize,
}

impl<L: Readable, R: Readable> Executable for JmpLeU<L, R> {
    fn execute(&self, state: &mut crate::State) {
        let l_val = self.l.get_signed(state);
        let r_val = self.r.get_signed(state);
        if l_val <= r_val {
            state.registers.m.ins.jump(self.loc);
        }
    }
}

pub(crate) struct Call {
    loc: usize,
}

impl Executable for Call {
    fn execute(&self, state: &mut crate::State) {
        let i_val = state.registers.m.ins.get_unsigned();
        state.stack.push(i_val);
        state.registers.m.ins.jump(self.loc);
    }
}

pub(crate) struct Ret<S: Readable> {
    s: S,
}

impl<S: Readable> Executable for Ret<S> {
    fn execute(&self, state: &mut crate::State) {
        state
            .registers
            .m
            .ret
            .set_unsigned(self.s.get_unsigned(state));
        let target_jump_u64 = state.stack.pop().expect("stack is empty, couldn't return");
        let target_jump =
            usize::try_from(target_jump_u64).expect("runtime archtiecture is to small");
        state.registers.m.ins.jump(target_jump);
    }
}

pub(super) fn transpile_control_flow(instr: ControlFlow) -> Box<dyn Executable> {
    match instr {
        ControlFlow::Jmp { label } => Box::new(Jmp {
            loc: label.label.unwrap().loc,
        }),
        ControlFlow::JmpEq { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpEq {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpNe { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpNe {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpGtS { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpGtS {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpGeS { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpGeS {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpLtS { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpLtS {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpLeS { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpLeS {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpGtU { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpGtU {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpGeU { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpGeU {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpLtU { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpLtU {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::JmpLeU { l, r, label } => {
            let l = RegisterSpecifier::from(l);
            let r = RegisterSpecifier::from(r);
            Box::new(JmpLeU {
                l,
                r,
                loc: label.label.unwrap().loc,
            })
        }
        ControlFlow::Cal { label } => Box::new(Call {
            loc: label.label.unwrap().loc,
        }),
        ControlFlow::Ret { s } => {
            let s = RegisterSpecifier::from(s);
            Box::new(Ret { s })
        }
    }
}
