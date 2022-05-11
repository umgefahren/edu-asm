use crate::{token::PureToken, Parsable, SelfParsed, Span};
use lazy_static::lazy_static;
use regex::Regex;

const LABEL_REF_EXP: &str = ":([A-Za-z]+)";

lazy_static! {
    static ref LABEL_REF_RE: Regex = Regex::new(LABEL_REF_EXP).unwrap();
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelRefToken {
    pub content: String,
}

impl From<LabelRefToken> for PureToken {
    #[inline]
    fn from(lt: LabelRefToken) -> Self {
        Self::LabelRef(lt)
    }
}

impl Parsable for LabelRefToken {
    fn parse(inp: &str) -> Vec<SelfParsed<Self>> {
        LABEL_REF_RE
            .captures_iter(inp)
            .map(|e| {
                let mut group_iter = e.iter();
                let whole_match = group_iter.next().unwrap().unwrap();
                let start = whole_match.start();
                let end = whole_match.end();
                let span = Span { start, end };
                let content = group_iter.next().unwrap().unwrap().as_str().to_string();
                let token = LabelRefToken { content };
                SelfParsed { inner: token, span }
            })
            .collect()
    }
}
