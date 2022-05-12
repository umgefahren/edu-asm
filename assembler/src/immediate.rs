use edu_asm_parser::{instruction::RegisterOrLiteral, literal::LiteralToken};
use tinyvec::ArrayVec;

use crate::register::encode_register_token;

pub fn encode_immediate(literal: &LiteralToken) -> [u8; 8] {
    match literal {
        LiteralToken::Signed(value) => value.to_le_bytes(),
        LiteralToken::Unsigned(value) => value.to_le_bytes(),
    }
}

pub fn encode_immediate_or_register(inp: &RegisterOrLiteral) -> ArrayVec<[u8; 8]> {
    let mut ret = ArrayVec::new();

    match inp {
        RegisterOrLiteral::Literal(l) => {
            let encoded = encode_immediate(l);
            ret.extend(encoded);
        }
        RegisterOrLiteral::Register(r) => {
            ret.push(encode_register_token(r));
        }
    }

    ret
}
