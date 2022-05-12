use std::{str::FromStr, rc::Rc};

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::label::LocAwLabel;

const LABEL_REF_EXP: &str = ":([A-Za-z]+)";

lazy_static! {
    static ref LABEL_REF_RE: Regex = Regex::new(LABEL_REF_EXP).unwrap();
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelRefToken {
    pub content: String,
    pub label: Option<Rc<LocAwLabel>>,
}

#[derive(Debug, Error)]
pub enum LabelRefParseError {
    #[error("invalid formatted label ref")]
    InvalidFormatted(String),
}

impl FromStr for LabelRefToken {
    type Err = LabelRefParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match LABEL_REF_RE.captures(s) {
            Some(d) => {
                let whole_match = d.get(0).unwrap();
                if whole_match.as_str() != s {
                    return Err(LabelRefParseError::InvalidFormatted(s.to_string()));
                }
                let inner_content = d.get(1).unwrap().as_str().to_string();
                Ok(Self {
                    content: inner_content,
                    label: None
                })
            }
            None => Err(LabelRefParseError::InvalidFormatted(s.to_string())),
        }
    }
}

