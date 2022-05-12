use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

const LABEL_EXP: &str = "([A-Za-z]+):";

lazy_static! {
    static ref LABEL_RE: Regex = Regex::new(LABEL_EXP).unwrap();
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelToken {
    pub content: String,
}

impl Clone for LabelToken {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.content.clone_from(&source.content)
    }
}

#[derive(Error, Debug)]
pub enum LabelParseError {
    #[error("the label string `{0}` is invalid")]
    InvalidFormatted(String),
}

impl FromStr for LabelToken {
    type Err = LabelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match LABEL_RE.captures(s) {
            Some(d) => {
                let whole_match = d.get(0).unwrap();
                if whole_match.as_str() != s {
                    return Err(LabelParseError::InvalidFormatted(s.to_string()));
                }
                let inner_content = d.get(1).unwrap().as_str().to_string();

                Ok(Self {
                    content: inner_content,
                })
            }
            None => Err(LabelParseError::InvalidFormatted(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocAwLabel {
    pub name: String,
    pub loc: usize,
}

impl LocAwLabel {
    #[inline]
    pub(crate) fn new(name: String, loc: usize) -> Self {
        Self { name, loc }
    }
}
