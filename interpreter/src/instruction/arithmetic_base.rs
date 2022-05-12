use edu_asm_parser::instruction::ArithmeticBase;

use crate::{behaviour::{Writeable, Readable}, register::RegisterSpecifier};

use super::{Executable, RegOrLit};

pub(crate) struct AddTs<D: Writeable, S: Readable, T: Readable>{
    d: D,
    s: S,
    t: T,
}

impl<D: Writeable, S: Readable, T: Readable> Executable for AddTs<D, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_signed(state);
        let t_val = self.t.get_signed(state);
        let d_val = s_val + t_val;
        self.d.set_signed(state, d_val);
    }
}

pub(crate) struct AddIs<S: Writeable + Readable, T: Readable> {
    s: S,
    t: T,
}

impl<S: Writeable + Readable, T: Readable> Executable for AddIs<S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_signed(state);
        let t_val = self.t.get_signed(state);
        let tmp = s_val + t_val;
        self.s.set_signed(state, tmp);
    }
}

pub(crate) struct AddTu<D: Writeable, S: Readable, T: Readable> {
    d: D,
    s: S,
    t: T
}

impl<D: Writeable, S: Readable, T: Readable> Executable for AddTu<D, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_unsigned(state);
        let t_val = self.t.get_unsigned(state);
        let d_val = s_val + t_val;
        self.d.set_unsigned(state, d_val);
    }
}

pub(crate) struct AddIu<S: Writeable + Readable, T: Readable> {
    s: S,
    t: T,
}

impl<S: Writeable + Readable, T: Readable> Executable for AddIu<S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_unsigned(state);
        let t_val = self.t.get_unsigned(state);
        let tmp = s_val + t_val;
        self.s.set_unsigned(state, tmp);
    }
}

pub(crate) struct SubTs<D: Writeable, S: Readable, T: Readable>{
    d: D,
    s: S,
    t: T,
}

impl<D: Writeable, S: Readable, T: Readable> Executable for SubTs<D, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_signed(state);
        let t_val = self.t.get_signed(state);
        let d_val = s_val - t_val;
        self.d.set_signed(state, d_val);
    }
}

pub(crate) struct SubIs<S: Writeable + Readable, T: Readable> {
    s: S,
    t: T,
}

impl<S: Writeable + Readable, T: Readable> Executable for SubIs<S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_signed(state);
        let t_val = self.t.get_signed(state);
        let tmp = s_val - t_val;
        self.s.set_signed(state, tmp);
    }
}

pub(crate) struct SubTu<D: Writeable, S: Readable, T: Readable> {
    d: D,
    s: S,
    t: T
}

impl<D: Writeable, S: Readable, T: Readable> Executable for SubTu<D, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_unsigned(state);
        let t_val = self.t.get_unsigned(state);
        let d_val = s_val - t_val;
        self.d.set_unsigned(state, d_val);
    }
}

pub(crate) struct SubIu<S: Writeable + Readable, T: Readable> {
    s: S,
    t: T,
}

impl<S: Writeable + Readable, T: Readable> Executable for SubIu<S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_unsigned(state);
        let t_val = self.t.get_unsigned(state);
        let tmp = s_val - t_val;
        self.s.set_unsigned(state, tmp);
    }
}

#[inline]
pub(super) fn transpile_arithmetic_base(instr: ArithmeticBase) -> Box<dyn Executable> {
    match instr {
        ArithmeticBase::AddTs { d , s, t } => {
            let d_spec = RegisterSpecifier::from(d);
            let s_var: RegOrLit = s.into();
            let t_var: RegOrLit = t.into();
            Box::new(AddTs {
                d: d_spec,
                s: s_var,
                t: t_var,
            })
        },
        ArithmeticBase::AddIs { s, t } => {
            let d_spec: RegisterSpecifier = s.into();
            let t_var: RegOrLit = t.into();
            Box::new(AddIs {
                s: d_spec,
                t: t_var
            })
        },
        ArithmeticBase::AddTu { d: d_t, s: s_t, t: t_t } => {
            let d: RegisterSpecifier = d_t.into();
            let s: RegOrLit = s_t.into();
            let t: RegOrLit = t_t.into();
            Box::new(AddTu {
                d,
                s,
                t
            })
        },
        ArithmeticBase::AddIu { s: s_t, t: t_t } => {
            let s: RegisterSpecifier = s_t.into();
            let t: RegOrLit = t_t.into();
            Box::new(AddIu {
                s,
                t
            })
        },
        ArithmeticBase::SubTs { d , s, t } => {
            let d_spec = RegisterSpecifier::from(d);
            let s_var: RegOrLit = s.into();
            let t_var: RegOrLit = t.into();
            Box::new(SubTs {
                d: d_spec,
                s: s_var,
                t: t_var,
            })
        },
        ArithmeticBase::SubIs { s, t } => {
            let d_spec: RegisterSpecifier = s.into();
            let t_var: RegOrLit = t.into();
            Box::new(SubIs {
                s: d_spec,
                t: t_var
            })
        },
        ArithmeticBase::SubTu { d: d_t, s: s_t, t: t_t } => {
            let d: RegisterSpecifier = d_t.into();
            let s: RegOrLit = s_t.into();
            let t: RegOrLit = t_t.into();
            Box::new(SubTu {
                d,
                s,
                t
            })
        },
        ArithmeticBase::SubIu { s: s_t, t: t_t } => {
            let s: RegisterSpecifier = s_t.into();
            let t: RegOrLit = t_t.into();
            Box::new(SubIu {
                s,
                t
            })
        }
    }
}
