use std::rc::Rc;
use std::str::FromStr;

use thiserror::Error;

use crate::{
    label::LocAwLabel,
    label_ref::{LabelRefParseError, LabelRefToken},
    literal::{LiteralParseError, LiteralToken},
    register::{RegisterParseError, RegisterToken},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    ArithmeticBase(ArithmeticBase),
    ArithmeticMultDivEasy(ArithmeticMultDivEasy),
    ArithmeticShift(ArithmeticShift),
    ArithmeticBitLogic(ArithmeticBitLogic),
    ControlFlow(ControlFlow),
    Memory(Memory),
    Misc(Misc),
}

#[derive(Debug, Error)]
pub enum InstructionParseError {
    #[error("parsing register failed")]
    RegisterParseError(#[from] RegisterParseError),
    #[error("parsing literal failed")]
    LiteralParseError(#[from] LiteralParseError),
    #[error("parsing as register failed with `{0}`, parsing as literal failed with `{1}`")]
    RegisterLiteralParseError(RegisterParseError, LiteralParseError),
    #[error("parsing label ref failed")]
    LabelRefParseError(#[from] LabelRefParseError),
    #[error("instruction `{0}` is unknown")]
    UnknownInstruction(String),
    #[error("label `{0}` in instruction `{1}` not found")]
    UnknownLabel(String, String),
}

impl InstructionParseError {
    pub(crate) fn is_unknown_instruction(&self) -> bool {
        matches!(self, InstructionParseError::UnknownInstruction(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegisterOrLiteral {
    Register(RegisterToken),
    Literal(LiteralToken),
}

impl FromStr for RegisterOrLiteral {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match RegisterToken::from_str(s) {
            Ok(d) => Ok(RegisterOrLiteral::Register(d)),
            Err(e) => match LiteralToken::from_str(s) {
                Ok(d) => Ok(RegisterOrLiteral::Literal(d)),
                Err(l) => Err(InstructionParseError::RegisterLiteralParseError(e, l)),
            },
        }
    }
}

impl From<RegisterToken> for RegisterOrLiteral {
    fn from(r: RegisterToken) -> Self {
        RegisterOrLiteral::Register(r)
    }
}

impl From<LiteralToken> for RegisterOrLiteral {
    fn from(l: LiteralToken) -> Self {
        RegisterOrLiteral::Literal(l)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticBase {
    AddTs {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    AddIs {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    AddTu {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    AddIu {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    SubTs {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    SubIs {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    SubTu {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    SubIu {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
}

#[inline]
fn split_collect(inp: &str) -> Vec<&str> {
    inp.split_whitespace().collect()
}

impl FromStr for ArithmeticBase {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_collect(s);

        match parts[..] {
            ["addts", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::AddTs { d, s, t })
            }
            ["addis", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::AddIs { s, t })
            }
            ["addtu", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::AddTu { d, s, t })
            }
            ["addiu", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::AddIu { s, t })
            }
            ["subts", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::SubTs { d, s, t })
            }
            ["subis", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::SubIs { s, t })
            }
            ["subtu", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::SubTu { d, s, t })
            }
            ["subiu", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBase::SubIu { s, t })
            }
            _ => Err(InstructionParseError::UnknownInstruction(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticShift {
    LshLT {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    LshLI {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    LshRT {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    LshRI {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    AshRT {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    AshRI {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
}

impl FromStr for ArithmeticShift {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_collect(s);

        match parts[..] {
            ["lshlt", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticShift::LshLT { d, s, t })
            }
            ["lshli", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticShift::LshLI { s, t })
            }
            ["lshrt", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticShift::LshRT { d, s, t })
            }
            ["lshri", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticShift::LshRI { s, t })
            }
            ["ashrt", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticShift::AshRT { d, s, t })
            }
            ["ashri", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticShift::AshRI { s, t })
            }
            _ => Err(InstructionParseError::UnknownInstruction(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticBitLogic {
    AndT {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    AndI {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    OrT {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    OrI {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    XorT {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    XorI {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    NotT {
        d: RegisterToken,
        s: RegisterOrLiteral,
    },
    NoI {
        s: RegisterToken,
    },
}

impl FromStr for ArithmeticBitLogic {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_collect(s);

        match parts[..] {
            ["andt", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBitLogic::AndT { d, s, t })
            }
            ["andi", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBitLogic::AndI { s, t })
            }
            ["ort", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBitLogic::OrT { d, s, t })
            }
            ["ori", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBitLogic::OrI { s, t })
            }
            ["xort", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBitLogic::XorT { d, s, t })
            }
            ["xori", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticBitLogic::XorI { s, t })
            }
            ["nott", d_str, s_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                Ok(ArithmeticBitLogic::NotT { d, s })
            }
            ["noi", s_str] => {
                let s = RegisterToken::from_str(s_str)?;
                Ok(ArithmeticBitLogic::NoI { s })
            }
            _ => Err(InstructionParseError::UnknownInstruction(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticMultDivEasy {
    MulTsE {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    MulIsE {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    MulTuE {
        d: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    MulIuE {
        s: RegisterToken,
        t: RegisterOrLiteral,
    },
    DivTsE {
        d: RegisterToken,
        r: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    DivTuE {
        d: RegisterToken,
        r: RegisterToken,
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
}

impl FromStr for ArithmeticMultDivEasy {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_collect(s);

        match parts[..] {
            ["mults_e", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticMultDivEasy::MulTsE { d, s, t })
            }
            ["mulis_e", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticMultDivEasy::MulIsE { s, t })
            }
            ["multu_e", d_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticMultDivEasy::MulTuE { d, s, t })
            }
            ["muliu_e", s_str, t_str] => {
                let s = RegisterToken::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticMultDivEasy::MulIuE { s, t })
            }
            ["divts_e", d_str, r_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticMultDivEasy::DivTsE { d, r, s, t })
            }
            ["divtu_e", d_str, r_str, s_str, t_str] => {
                let d = RegisterToken::from_str(d_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(ArithmeticMultDivEasy::DivTuE { d, r, s, t })
            }
            _ => Err(InstructionParseError::UnknownInstruction(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControlFlow {
    Jmp {
        label: LabelRefToken,
    },
    JmpEq {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpNe {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpGtS {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpGeS {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpLtS {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpLeS {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpGtU {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpGeU {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpLtU {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    JmpLeU {
        l: RegisterToken,
        r: RegisterToken,
        label: LabelRefToken,
    },
    Cal {
        label: LabelRefToken,
    },
    Ret {
        s: RegisterToken,
    },
}

impl ControlFlow {
    pub fn get_label(&self) -> Option<&LabelRefToken> {
        match self {
            ControlFlow::Jmp { label } => Some(label),
            ControlFlow::JmpEq { label, .. } => Some(label),
            ControlFlow::JmpNe { label, .. } => Some(label),
            ControlFlow::JmpGtS { label, .. } => Some(label),
            ControlFlow::JmpGeS { label, .. } => Some(label),
            ControlFlow::JmpLtS { label, .. } => Some(label),
            ControlFlow::JmpLeS { label, .. } => Some(label),
            ControlFlow::JmpGtU { label, .. } => Some(label),
            ControlFlow::JmpGeU { label, .. } => Some(label),
            ControlFlow::JmpLtU { label, .. } => Some(label),
            ControlFlow::JmpLeU { label, .. } => Some(label),
            ControlFlow::Cal { label, .. } => Some(label),
            ControlFlow::Ret { .. } => None,
        }
    }

    pub fn hydrate(&mut self, loc_label: Rc<LocAwLabel>) {
        match self {
            ControlFlow::Jmp { label } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpEq { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpNe { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpGtS { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpGeS { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpLtS { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpLeS { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpGtU { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpGeU { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpLtU { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::JmpLeU { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::Cal { label, .. } => {
                label.label = Some(loc_label);
            }
            ControlFlow::Ret { .. } => {}
        };
    }
}

impl FromStr for ControlFlow {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_collect(s);
        match parts[..] {
            ["jmp", label_str] => {
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::Jmp { label })
            }
            ["jmpeq", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpEq { l, r, label })
            }
            ["jmpne", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpNe { l, r, label })
            }
            ["jmpgts", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpGtS { l, r, label })
            }
            ["jmpges", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpGeS { l, r, label })
            }
            ["jmplts", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpLtS { l, r, label })
            }
            ["jmples", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpLeS { l, r, label })
            }
            ["jmpgtu", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpGtU { l, r, label })
            }
            ["jmpgeu", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpGeU { l, r, label })
            }
            ["jmpltu", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpLtU { l, r, label })
            }
            ["jmpleu", l_str, r_str, label_str] => {
                let l = RegisterToken::from_str(l_str)?;
                let r = RegisterToken::from_str(r_str)?;
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::JmpLeU { l, r, label })
            }
            ["cal", label_str] => {
                let label = LabelRefToken::from_str(label_str)?;
                Ok(ControlFlow::Cal { label })
            }
            ["ret", s_str] => {
                let s = RegisterToken::from_str(s_str)?;
                Ok(ControlFlow::Ret { s })
            }
            _ => Err(InstructionParseError::UnknownInstruction(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Memory {
    Mov {
        /// this is the target register
        t: RegisterToken,
        /// this is the source register or literal
        s: RegisterOrLiteral,
    },
    Load {
        t: RegisterToken,
        s: RegisterOrLiteral,
    },
    LoadO {
        t: RegisterToken,
        s: RegisterOrLiteral,
        o: RegisterOrLiteral,
    },
    Loadb {
        t: RegisterToken,
        s: RegisterOrLiteral,
    },
    LoadbO {
        t: RegisterToken,
        s: RegisterOrLiteral,
        o: RegisterOrLiteral,
    },
    Stor {
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    StorO {
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
        o: RegisterOrLiteral,
    },
    Storb {
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
    },
    StorbO {
        s: RegisterOrLiteral,
        t: RegisterOrLiteral,
        o: RegisterOrLiteral,
    },
    Push {
        d: RegisterToken,
    },
    Pop {
        d: RegisterToken,
    },
}

impl FromStr for Memory {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_collect(s);
        match parts[..] {
            ["mov", t_str, s_str] => {
                let t = RegisterToken::from_str(t_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                Ok(Memory::Mov { t, s })
            }
            ["load", t_str, s_str] => {
                let t = RegisterToken::from_str(t_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                Ok(Memory::Load { t, s })
            }
            ["loado", t_str, s_str, o_str] => {
                let t = RegisterToken::from_str(t_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let o = RegisterOrLiteral::from_str(o_str)?;
                Ok(Memory::LoadO { t, s, o })
            }
            ["loadb", t_str, s_str] => {
                let t = RegisterToken::from_str(t_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                Ok(Memory::Loadb { t, s })
            }
            ["loadbo", t_str, s_str, o_str] => {
                let t = RegisterToken::from_str(t_str)?;
                let s = RegisterOrLiteral::from_str(s_str)?;
                let o = RegisterOrLiteral::from_str(o_str)?;
                Ok(Memory::LoadbO { t, s, o })
            }
            ["stor", s_str, t_str] => {
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(Memory::Stor { s, t })
            }
            ["storo", s_str, t_str, o_str] => {
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                let o = RegisterOrLiteral::from_str(o_str)?;
                Ok(Memory::StorO { s, t, o })
            }
            ["storb", s_str, t_str] => {
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                Ok(Memory::Storb { s, t })
            }
            ["storbo", s_str, t_str, o_str] => {
                let s = RegisterOrLiteral::from_str(s_str)?;
                let t = RegisterOrLiteral::from_str(t_str)?;
                let o = RegisterOrLiteral::from_str(o_str)?;
                Ok(Memory::StorbO { s, t, o })
            }
            ["push", d_str] => {
                let d = RegisterToken::from_str(d_str)?;
                Ok(Memory::Push { d })
            }
            ["pop", d_str] => {
                let d = RegisterToken::from_str(d_str)?;
                Ok(Memory::Pop { d })
            }
            _ => Err(InstructionParseError::UnknownInstruction(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Misc {
    Halt,
    Exit { s: RegisterOrLiteral },
    Print { s: RegisterOrLiteral },
    Read { s: RegisterToken },
    Dump,
    Nop,
}

// I crave for metaprogramming

impl FromStr for Misc {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_collect(s);
        match parts[..] {
            ["halt"] => Ok(Misc::Halt),
            ["exit", s_str] => {
                let s = RegisterOrLiteral::from_str(s_str)?;
                Ok(Misc::Exit { s })
            }
            ["print", s_str] => {
                let s = RegisterOrLiteral::from_str(s_str)?;
                Ok(Misc::Print { s })
            }
            ["read", s_str] => {
                let s = RegisterToken::from_str(s_str)?;
                Ok(Misc::Read { s })
            }
            ["dump"] => Ok(Misc::Dump),
            ["nop"] => Ok(Misc::Nop),
            _ => Err(InstructionParseError::UnknownInstruction(s.to_string())),
        }
    }
}
