use edu_asm_parser::{instruction::{
    ArithmeticBase, ArithmeticMultDivEasy, Instruction, RegisterOrLiteral, ArithmeticShift, ArithmeticBitLogic, ControlFlow, Memory, Misc,
}, label_ref::LabelRefToken, literal::LiteralToken};

use crate::{
    immediate::encode_immediate_or_register,
    mode::{register_or_literal_slice, OperationMode}, instruction_ident::inst::*,
};

pub fn encode_instruction_parameter(parameters: &[RegisterOrLiteral]) -> Vec<u8> {
    assert!(parameters.len() <= 8);

    let mut ret = Vec::with_capacity(parameters.len() + 1);

    let params = register_or_literal_slice(parameters);

    let mode = OperationMode::new(&params);
    let encoded_mode = mode.encode();

    ret.push(encoded_mode);

    parameters.iter().for_each(|e| {
        let bytes = encode_immediate_or_register(e);
        ret.extend(&bytes);
    });

    ret.shrink_to_fit();

    ret
}

#[inline]
pub fn encode_three<
    A: Into<RegisterOrLiteral>,
    B: Into<RegisterOrLiteral>,
    C: Into<RegisterOrLiteral>,
>(
    a: A,
    b: B,
    c: C,
) -> Vec<u8> {
    let a = a.into();
    let b = b.into();
    let c = c.into();
    let parameters = &[a, b, c];
    encode_instruction_parameter(parameters)
}

#[inline(always)]
pub fn encode_two<A: Into<RegisterOrLiteral>, B: Into<RegisterOrLiteral>>(a: A, b: B) -> Vec<u8> {
    let a = a.into();
    let b = b.into();
    let parameters = &[a, b];
    encode_instruction_parameter(parameters)
}

#[inline(always)]
pub fn encode_four<
    A: Into<RegisterOrLiteral>,
    B: Into<RegisterOrLiteral>,
    C: Into<RegisterOrLiteral>,
    D: Into<RegisterOrLiteral>,
>(
    a: A,
    b: B,
    c: C,
    d: D,
) -> Vec<u8> {
    let a = a.into();
    let b = b.into();
    let c = c.into();
    let d = d.into();
    let parameters = &[a, b, c, d];
    encode_instruction_parameter(parameters)
}

#[inline(always)]
pub fn build_complete_instruction(instruction_ident: [u8; 3], parameters: Vec<u8>) -> Vec<u8> {
    let mut ret = Vec::with_capacity(parameters.len() + 3);

    ret.extend(&instruction_ident);
    ret.extend(&parameters);

    ret.shrink_to_fit();

    ret
}

#[inline(always)]
pub fn encode_label_ref(label_ref: LabelRefToken) -> RegisterOrLiteral { 
    let loc_usize = label_ref.label.unwrap().loc;
    let loc = u64::try_from(loc_usize).unwrap(); 
    RegisterOrLiteral::Literal(LiteralToken::Unsigned(loc))
}

#[inline(always)]
pub fn encode_arithmetic_base(instruction: ArithmeticBase) -> Vec<u8> {
    match instruction {
        ArithmeticBase::AddTs { d, s, t } => build_complete_instruction(ADD_T_S_INST, encode_three(d, s, t)),
        ArithmeticBase::AddIs { s, t } => build_complete_instruction(ADD_I_S_INST, encode_two(s, t)),
        ArithmeticBase::AddTu { d, s, t } => build_complete_instruction(ADD_T_U_INST, encode_three(d, s, t)),
        ArithmeticBase::AddIu { s, t } => build_complete_instruction(ADD_I_U_INST, encode_two(s, t)),
        ArithmeticBase::SubTs { d, s, t } => build_complete_instruction(SUB_T_S_INST, encode_three(d, s, t)),
        ArithmeticBase::SubIs { s, t } => build_complete_instruction(SUB_I_S_INST, encode_two(s, t)),
        ArithmeticBase::SubTu { d, s, t } => build_complete_instruction(SUB_T_U_INST, encode_three(d, s, t)),
        ArithmeticBase::SubIu { s, t } => build_complete_instruction(SUB_I_U_INST, encode_two(s, t)),
    }
}

#[inline(always)]
pub fn encode_arithmetic_shift(instruction: ArithmeticShift) -> Vec<u8> {
    match instruction {
        ArithmeticShift::LshLT { d, s, t } => build_complete_instruction(LSHL_T_S_INST, encode_three(d, s, t)),
        ArithmeticShift::LshLI { s, t } => build_complete_instruction(LSHL_I_S_INST, encode_two(s, t)),
        ArithmeticShift::LshRT { d, s, t } => build_complete_instruction(LSHR_T_S_INST, encode_three(d, s, t)),
        ArithmeticShift::LshRI { s, t } => build_complete_instruction(LSHR_I_S_INST, encode_two(s, t)),
        ArithmeticShift::AshRT { d, s, t } => build_complete_instruction(ASHR_T_S_INST, encode_three(d, s, t)),
        ArithmeticShift::AshRI { s, t } => build_complete_instruction(ASHR_I_S_INST, encode_two(s, t)),
    }
}

#[inline(always)]
pub fn encode_arithmetic_bit_log(instruction: ArithmeticBitLogic) -> Vec<u8> {
    match instruction {
        ArithmeticBitLogic::AndT { d, s, t } => build_complete_instruction(AND_T_S_INST, encode_three(d, s, t)),
        ArithmeticBitLogic::AndI { s, t } => build_complete_instruction(AND_I_S_INST, encode_two(s, t)),
        ArithmeticBitLogic::OrT { d, s, t } => build_complete_instruction(OR_T_S_INST, encode_three(d, s, t)),
        ArithmeticBitLogic::OrI { s, t } => build_complete_instruction(OR_I_S_INST, encode_two(s, t)),
        ArithmeticBitLogic::XorT { d, s, t } => build_complete_instruction(XOR_T_S_INST, encode_three(d, s, t)),
        ArithmeticBitLogic::XorI { s, t } => build_complete_instruction(XOR_I_S_INST, encode_two(s, t)),
        ArithmeticBitLogic::NotT { d, s } => build_complete_instruction(NOT_T_S_INST, encode_two(d, s)),
        ArithmeticBitLogic::NoI { s } => build_complete_instruction(NOT_I_S_INST, encode_instruction_parameter(&[s.into()])),
    }
}

#[inline(always)]
pub fn encode_arithmetic_mult_div(instruction: ArithmeticMultDivEasy) -> Vec<u8> {
    match instruction {
        ArithmeticMultDivEasy::MulTsE { d, s, t } => build_complete_instruction(MUL_T_S_E_INST, encode_three(d, s, t)),
        ArithmeticMultDivEasy::MulIsE { s, t } => build_complete_instruction(MUL_I_S_E_INST, encode_two(s, t)),
        ArithmeticMultDivEasy::MulTuE { d, s, t } => build_complete_instruction(MUL_T_U_E_INST, encode_three(d, s, t)),
        ArithmeticMultDivEasy::MulIuE { s, t } => build_complete_instruction(MUL_I_U_E_INST, encode_two(s, t)),
        ArithmeticMultDivEasy::DivTsE { d, r, s, t } => build_complete_instruction(DIV_T_S_E_INST, encode_four(d, r, s, t)),
        ArithmeticMultDivEasy::DivTuE { d, r, s, t } => build_complete_instruction(DIV_T_U_E_INST, encode_four(d, r, s, t)),
    }
}

#[inline(always)]
pub fn encode_control_flow(instruction: ControlFlow) -> Vec<u8> {
    match instruction {
        ControlFlow::Jmp { label } => build_complete_instruction(JMP_INST, encode_instruction_parameter(&[encode_label_ref(label)])),
        ControlFlow::JmpEq { l, r, label } => build_complete_instruction(JMP_EQ_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpNe { l, r, label } => build_complete_instruction(JMP_NE_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpGtS { l, r, label } => build_complete_instruction(JMP_GT_S_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpGeS { l, r, label } => build_complete_instruction(JMP_GE_S_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpLtS { l, r, label } => build_complete_instruction(JMP_LT_S_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpLeS { l, r, label } => build_complete_instruction(JMP_LE_S_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpGtU { l, r, label } => build_complete_instruction(JMP_GT_U_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpGeU { l, r, label } => build_complete_instruction(JMP_GE_U_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpLtU { l, r, label } => build_complete_instruction(JMP_LT_U_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::JmpLeU { l, r, label } => build_complete_instruction(JMP_LE_U_INST, encode_three(l, r, encode_label_ref(label))),
        ControlFlow::Cal { label } => build_complete_instruction(CAL_INST, encode_instruction_parameter(&[encode_label_ref(label)])),
        ControlFlow::Ret { s } => build_complete_instruction(RET_INST, encode_instruction_parameter(&[s.into()])),
    }
}

#[inline(always)]
pub fn encode_memory(instruction: Memory) -> Vec<u8> {
    match instruction {
        Memory::Mov { t, s } => build_complete_instruction(MOV_INST, encode_two(t, s)),
        Memory::Load { t, s } => build_complete_instruction(LOAD_INST, encode_two(t, s)),
        Memory::LoadO { t, s, o } => build_complete_instruction(LOAD_O_INST, encode_three(t, s, o)),
        Memory::Loadb { t, s } => build_complete_instruction(LOAD_B_INST, encode_two(t, s)),
        Memory::LoadbO { t, s, o } => build_complete_instruction(LOAD_B_O_INST, encode_three(t, s, o)),
        Memory::Stor { t, s } => build_complete_instruction(STOR_INST, encode_two(t, s)),
        Memory::StorO { t, s, o } => build_complete_instruction(STOR_O_INST, encode_three(t, s, o)),
        Memory::Storb { t, s } => build_complete_instruction(STOR_B_INST, encode_two(t, s)),
        Memory::StorbO { t, s, o } => build_complete_instruction(STOR_B_O_INST, encode_three(t, s, o)),
        Memory::Push { d } => build_complete_instruction(PUSH_INST, encode_instruction_parameter(&[d.into()])),
        Memory::Pop { d } => build_complete_instruction(POP_INST, encode_instruction_parameter(&[d.into()])),
    }
}

#[inline(always)]
pub fn encode_misc(insruction: Misc) -> Vec<u8> {
    match insruction {
        Misc::Halt => build_complete_instruction(HALT_INST, vec![]),
        Misc::Exit { s } => build_complete_instruction(EXIT_INST, encode_instruction_parameter(&[s])),
        Misc::Print { s } => build_complete_instruction(PRINT_INST, encode_instruction_parameter(&[s])),
        Misc::Read { s } => build_complete_instruction(READ_INST, encode_instruction_parameter(&[s])),
        Misc::Dump => build_complete_instruction(DUMP_INST, vec![]),
        Misc::Nop => build_complete_instruction(NOP_INST, vec![]),
    }
}

#[inline]
pub fn encode_instruction(instruction: Instruction) -> Vec<u8> {
    match instruction {
        Instruction::ArithmeticBase(b) => encode_arithmetic_base(b),
        Instruction::ArithmeticShift(s) => encode_arithmetic_shift(s),
        Instruction::ArithmeticBitLogic(b) => encode_arithmetic_bit_log(b),
        Instruction::ArithmeticMultDivEasy(m) => encode_arithmetic_mult_div(m),
        Instruction::ControlFlow(c) => encode_control_flow(c),
        Instruction::Memory(m) => encode_memory(m),
        Instruction::Misc(m) => encode_misc(m),
    }
}
