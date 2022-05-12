use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegisterToken {
    GeneralPurpose(u8),
    StackBase,
    StackEnd,
    Return,
    Instruction,
    Zero,
    Error,
}

#[derive(Error, Debug)]
pub enum RegisterParseError {
    #[error("the register string `{0}` is invalid")]
    InvalidFormatted(String),
}

impl FromStr for RegisterToken {
    type Err = RegisterParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = match s {
            "$G_0" => RegisterToken::GeneralPurpose(0),
            "$G_1" => RegisterToken::GeneralPurpose(1),
            "$G_2" => RegisterToken::GeneralPurpose(2),
            "$G_3" => RegisterToken::GeneralPurpose(3),
            "$G_4" => RegisterToken::GeneralPurpose(4),
            "$G_5" => RegisterToken::GeneralPurpose(5),
            "$G_6" => RegisterToken::GeneralPurpose(6),
            "$G_7" => RegisterToken::GeneralPurpose(7),
            "$S_B" => RegisterToken::StackBase,
            "$S_E" => RegisterToken::StackEnd,
            "$R" => RegisterToken::Return,
            "$I" => RegisterToken::Instruction,
            "$Z" => RegisterToken::Zero,
            "$E" => RegisterToken::Error,
            _ => {
                return Err(RegisterParseError::InvalidFormatted(s.to_string()));
            }
        };
        Ok(ret)
    }
}
