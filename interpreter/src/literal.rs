use edu_asm_parser::literal::LiteralToken;

use crate::behaviour::Readable;

#[derive(Clone, Copy)]
pub(crate) union Literal {
    signed: i64,
    unsigned: u64,
}

impl From<LiteralToken> for Literal {
    fn from(lt: LiteralToken) -> Self {
        match lt {
            LiteralToken::Signed(signed) => Self { signed },
            LiteralToken::Unsigned(unsigned) => Self { unsigned },
        }
    }
}

impl Readable for Literal {
    #[inline]
    fn get_signed(&self, _: &crate::State) -> i64 {
        unsafe { self.signed }
    }

    #[inline]
    fn get_unsigned(&self, _: &crate::State) -> u64 {
        unsafe { self.unsigned }
    }
}
