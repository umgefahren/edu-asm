use edu_asm_parser::instruction::ArithmeticMultDivEasy;

use crate::{behaviour::{Writeable, Readable}, register::RegisterSpecifier};

use super::{Executable, RegOrLit};

pub(crate) struct MulTsE<D: Writeable, S: Readable, T: Readable> {
    d: D,
    s: S,
    t: T
}

impl<D: Writeable, S: Readable, T: Readable> Executable for MulTsE<D, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_signed(state);
        let t_val = self.t.get_signed(state);
        let d_val = s_val * t_val;
        self.d.set_signed(state, d_val);
    }
}

pub(crate) struct MulIsE<S: Writeable + Readable, T: Readable> {
    s: S,
    t: T,
}

impl<S: Writeable + Readable, T: Readable> Executable for MulIsE<S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_signed(state);
        let t_val = self.t.get_signed(state);
        let tmp = s_val * t_val;
        self.s.set_signed(state, tmp);
    }
}

pub(crate) struct MulTuE<D: Writeable, S: Readable, T: Readable> {
    d: D,
    s: S,
    t: T
}

impl<D: Writeable, S: Readable, T: Readable> Executable for MulTuE<D, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_unsigned(state);
        let t_val = self.t.get_unsigned(state);
        let d_val = s_val * t_val;
        self.d.set_unsigned(state, d_val);
    }
}

pub(crate) struct MulIuE<S: Writeable + Readable, T: Readable> {
    s: S,
    t: T,
}

impl<S: Writeable + Readable, T: Readable> Executable for MulIuE<S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_unsigned(state);
        let t_val = self.t.get_unsigned(state);
        let tmp = s_val * t_val;
        self.s.set_unsigned(state, tmp);
    }
}

pub(crate) struct DivTsE<D: Writeable, R: Writeable, S: Readable, T: Readable> {
    d: D,
    r: R,
    s: S,
    t: T,
}

impl<D: Writeable, R: Writeable, S: Readable, T: Readable> Executable for DivTsE<D, R, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_signed(state);
        let t_val = self.t.get_signed(state);
        let r_val = s_val % t_val;
        let d_val = s_val / t_val;
        self.r.set_signed(state, r_val);
        self.d.set_signed(state, d_val);
    }
}

pub(crate) struct DivTuE<D: Writeable, R: Writeable, S: Readable, T: Readable> {
    d: D,
    r: R,
    s: S,
    t: T,
}

impl<D: Writeable, R: Writeable, S: Readable, T: Readable> Executable for DivTuE<D, R, S, T> {
    fn execute(&self, state: &mut crate::State) {
        let s_val = self.s.get_unsigned(state);
        let t_val = self.t.get_unsigned(state);
        let r_val = s_val % t_val;
        let d_val = s_val / t_val;
        self.r.set_unsigned(state, r_val);
        self.d.set_unsigned(state, d_val);
    }
}

#[inline]
pub(super) fn transpile_arithmetic_mult_div(instr: ArithmeticMultDivEasy) -> Box<dyn Executable> {
    match instr {
        ArithmeticMultDivEasy::MulTsE { d, s, t } => {
            let d = RegisterSpecifier::from(d);
            let s = RegOrLit::from(s);
            let t = RegOrLit::from(t);
            Box::new(MulTsE {
                d,
                s,
                t
            })
        },
        ArithmeticMultDivEasy::MulIsE { s, t } => {
            let s = RegisterSpecifier::from(s);
            let t = RegOrLit::from(t);
            Box::new(MulIsE {
                s,
                t
            })
        },
        ArithmeticMultDivEasy::MulTuE { d, s, t } => {
            let d = RegisterSpecifier::from(d);
            let s = RegOrLit::from(s);
            let t = RegOrLit::from(t);
            Box::new(MulTuE {
                d,
                s,
                t
            })
        },
        ArithmeticMultDivEasy::MulIuE { s, t } => {
            let s = RegisterSpecifier::from(s);
            let t = RegOrLit::from(t);
            Box::new(MulIuE {
                s,
                t
            })
        },
        ArithmeticMultDivEasy::DivTsE { d, r, s, t } => {
            let d = RegisterSpecifier::from(d);
            let r = RegisterSpecifier::from(r);
            let s = RegOrLit::from(s);
            let t = RegOrLit::from(t);
            Box::new(DivTsE {
                d,
                r,
                s,
                t
            })
        },
        ArithmeticMultDivEasy::DivTuE { d, r, s, t } => {
            let d = RegisterSpecifier::from(d);
            let r = RegisterSpecifier::from(r);
            let s = RegOrLit::from(s);
            let t = RegOrLit::from(t);
            Box::new(DivTuE {
                d,
                r,
                s,
                t
            })
        },
    }
}
