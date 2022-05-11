use crate::{token::PureToken, Parsable, SelfParsed, Span};
use lazy_static::lazy_static;
use regex::Regex;

const COMMENT_EXP: &str = ";(.*)";

lazy_static! {
    static ref COMMENT_RE: Regex = Regex::new(COMMENT_EXP).unwrap();
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommentToken {
    pub content: String,
}

impl From<CommentToken> for PureToken {
    #[inline]
    fn from(ct: CommentToken) -> Self {
        Self::Comment(ct)
    }
}

impl Parsable for CommentToken {
    fn parse(inp: &str) -> Vec<SelfParsed<Self>> {
        COMMENT_RE
            .captures_iter(inp)
            .map(|e| {
                let mut group_iter = e.iter();
                let whole_match = group_iter.next().unwrap().unwrap();
                let start = whole_match.start();
                let end = whole_match.end();
                let span = Span { start, end };
                let content = group_iter.next().unwrap().unwrap().as_str().to_string();
                let token = CommentToken { content };
                SelfParsed { inner: token, span }
            })
            .collect()
    }
}
