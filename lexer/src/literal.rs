use crate::{token::PureToken, Parsable, SelfParsed, Span};
use lazy_static::lazy_static;
use regex::Regex;

const LITERAL_EXP: &str = "((-)?[0-9]+(u|s)?)";

lazy_static! {
    static ref LITERAL_RE: Regex = Regex::new(LITERAL_EXP).unwrap();
}


#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiteralToken {
    pub content: String,
}

impl From<LiteralToken> for PureToken {
    fn from(l: LiteralToken) -> Self {
        PureToken::Literal(l)
    }
}

impl Parsable for LiteralToken {
    fn parse(inp: &str) -> Vec<crate::SelfParsed<Self>> {
        LITERAL_RE
            .captures_iter(inp)
            .map(|e| {
                let mut group_iter = e.iter();
                let whole_match = group_iter.next().unwrap().unwrap();
                let start = whole_match.start();
                let end = whole_match.end();
                let span = Span { start, end };
                let content = group_iter.next().unwrap().unwrap().as_str().to_string();
                let token = LiteralToken { content };
                SelfParsed { inner: token, span }
            })
            .collect()
    }
}
