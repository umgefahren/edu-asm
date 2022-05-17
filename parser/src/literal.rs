use std::{num::IntErrorKind, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::error::Errors;

const LITERAL_EXP: &str = "(-?)([0-9]+)(u|s)?";

lazy_static! {
    static ref LITERAL_RE: Regex = Regex::new(LITERAL_EXP).unwrap();
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralToken {
    Signed(i64),
    Unsigned(u64),
}

#[derive(Debug, Error)]
pub enum LiteralParseError {
    #[error("sign before literal `{0}` conflicts with literal type `{1}`")]
    ConflictingSignAndLiteralType(String, char),
    #[error("literal is invalid formatted")]
    InvalidFormatted(String),
    #[error("signed literal `{0}` exceeds i64 bounds `{1}`")]
    SignedLiteralToBig(String, i64),
    #[error("unsigend literal `{0}` exceeds u64 bounds `{1}`")]
    UnsignedLiteralToBig(String, u64),
}

impl FromStr for LiteralToken {
    type Err = LiteralParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(error) = Errors::from_str(s) {
            let error_num = error as u64;
            return Ok(LiteralToken::Unsigned(error_num));
        }
        let cap = match LITERAL_RE.captures(s) {
            None => return Err(LiteralParseError::InvalidFormatted(s.to_string())),
            Some(d) => d,
        };

        let negative = matches!(cap.get(1).unwrap().as_str(), "-");
        let number_str = cap.get(2).unwrap().as_str();
        match cap.get(3) {
            Some(d) => match d.as_str() {
                "s" => parse_signed_literal(s, negative, number_str),
                "u" => {
                    if negative {
                        return Err(LiteralParseError::ConflictingSignAndLiteralType(
                            s.to_string(),
                            'u',
                        ));
                    }
                    let number = match u64::from_str(number_str) {
                        Ok(d) => d,
                        Err(e) => match e.kind() {
                            IntErrorKind::PosOverflow => {
                                return Err(LiteralParseError::UnsignedLiteralToBig(
                                    s.to_string(),
                                    u64::MAX,
                                ))
                            }
                            IntErrorKind::NegOverflow => {
                                return Err(LiteralParseError::UnsignedLiteralToBig(
                                    s.to_string(),
                                    u64::MIN,
                                ))
                            }
                            _ => panic!("this shouldn't have happened"),
                        },
                    };
                    Ok(LiteralToken::Unsigned(number))
                }
                _ => panic!("this shouldn't be here"),
            },
            None => parse_signed_literal(s, negative, number_str),
        }
    }
}

fn parse_signed_literal(
    inp: &str,
    negative: bool,
    number_str: &str,
) -> Result<LiteralToken, LiteralParseError> {
    let mut number = match i64::from_str(number_str) {
        Ok(d) => d,
        Err(e) => match e.kind() {
            IntErrorKind::PosOverflow => {
                return Err(LiteralParseError::SignedLiteralToBig(
                    inp.to_string(),
                    i64::MAX,
                ))
            }
            IntErrorKind::NegOverflow => {
                return Err(LiteralParseError::SignedLiteralToBig(
                    inp.to_string(),
                    i64::MIN,
                ))
            }
            _ => panic!("this shouldn't have happened"),
        },
    };
    if negative {
        number *= -1;
    }
    Ok(LiteralToken::Signed(number))
}
