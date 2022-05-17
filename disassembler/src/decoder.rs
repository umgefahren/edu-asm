use std::io::{Cursor, Read};

use edu_asm_assembler::{
    instruction_ident::inst::*,
    mode::{OperationMode, RegisterLiteral},
    register::decode_register_token,
};
use edu_asm_parser::{
    instruction::{ArithmeticBase, Instruction, RegisterOrLiteral},
    literal::LiteralToken,
    register::RegisterToken,
};

pub fn decode_instruction(cursor: &mut Cursor<Vec<u8>>) -> Result<Instruction, ()> {
    let mut instruction_ident_buffer = [0u8; 3];
    cursor.read_exact(&mut instruction_ident_buffer).unwrap();
    match instruction_ident_buffer {
        ADD_T_S_INST => {
            const ADD_T_S_PARAMETERS: &[ParameterType] = &[
                ParameterType::Register,
                ParameterType::RegisterImmediate,
                ParameterType::RegisterImmediate,
            ];
            let parameters = decode_parameter(cursor, ADD_T_S_PARAMETERS)?;
            Ok(Instruction::ArithmeticBase(ArithmeticBase::AddTs {
                d: unwrap_register_or_literal(parameters[0]),
                s: parameters[1],
                t: parameters[2],
            }))
        }
        ADD_I_S_INST => {
            const ADD_I_S_PARAMETERS: &[ParameterType] = &[
                ParameterType::Register,
                ParameterType::RegisterImmediate
            ];
            let parameters = decode_parameter(cursor, ADD_I_S_PARAMETERS)?;
            Ok(Instruction::ArithmeticBase(ArithmeticBase::AddIs {
                s: unwrap_register_or_literal(parameters[0]),
                t: parameters[1],
            }))
        }
        _ => panic!("Unimplemented instruction"),
    }
}

#[allow(dead_code)]
enum ParameterType {
    Register,
    Immediate,
    RegisterImmediate,
}

fn unwrap_register_or_literal(register_or_literal: RegisterOrLiteral) -> RegisterToken {
    match register_or_literal {
        RegisterOrLiteral::Register(register) => register,
        RegisterOrLiteral::Literal(_) => panic!("Literal not supported"),
    }
}

fn read_register(cursor: &mut Cursor<Vec<u8>>) -> RegisterToken {
    let mut bin_buffer = [0u8; 1];
    cursor.read_exact(&mut bin_buffer).unwrap();
    decode_register_token(bin_buffer[0])
}

fn read_literal(cursor: &mut Cursor<Vec<u8>>) -> LiteralToken {
    let mut bin_buffer = [0u8; 8];
    cursor.read_exact(&mut bin_buffer).unwrap();
    LiteralToken::Unsigned(u64::from_le_bytes(bin_buffer))
}

fn decode_parameter(
    cursor: &mut Cursor<Vec<u8>>,
    parameters: &[ParameterType],
) -> Result<Vec<RegisterOrLiteral>, ()> {
    let mut operation_mode_buffer = [0u8; 1];
    cursor.read_exact(&mut operation_mode_buffer).unwrap();
    let operation_mode = OperationMode::decode(operation_mode_buffer[0]);
    let mut ret = Vec::with_capacity(parameters.len());
    for (idx, parameter) in parameters.iter().enumerate() {
        let parameter_op_mode = operation_mode.get(idx);
        match parameter {
            ParameterType::Register => assert!(
                parameter_op_mode.is_register(),
                "passed a literal where a register was required"
            ),
            ParameterType::Immediate => assert!(
                parameter_op_mode.is_literal(),
                "passed a register where a literal was required"
            ),
            _ => {}
        }
        match parameter_op_mode {
            RegisterLiteral::Register => {
                ret.push(RegisterOrLiteral::Register(read_register(cursor)))
            }
            RegisterLiteral::Literal => ret.push(RegisterOrLiteral::Literal(read_literal(cursor))),
        }
    }
    Ok(ret)
}
