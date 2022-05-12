use std::ops::Not;

use crate::{token::PureToken, Parsable, SelfParsed, Span};
use lazy_static::lazy_static;
use regex::Regex;

const OPERATOR_EXP: &str = "([A-Za-z]+(_e)?(:)?)";

lazy_static! {
    static ref OPERATOR_RE: Regex = Regex::new(OPERATOR_EXP).unwrap();
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperatorToken {
    pub content: String,
}

impl From<OperatorToken> for PureToken {
    #[inline]
    fn from(lt: OperatorToken) -> Self {
        Self::Operator(lt)
    }
}

impl Parsable for OperatorToken {
    fn parse(inp: &str) -> Vec<crate::SelfParsed<Self>> {
        OPERATOR_RE
            .captures_iter(inp)
            .filter(|e| e.get(0).unwrap().as_str().ends_with(':').not())
            .map(|e| {
                let mut group_iter = e.iter();
                let whole_match = group_iter.next().unwrap().unwrap();
                let start = whole_match.start();
                let end = whole_match.end();
                let span = Span { start, end };
                let content = group_iter.next().unwrap().unwrap().as_str().to_string();
                let token = OperatorToken { content };
                SelfParsed { inner: token, span }
            })
            .collect()
    }
}
