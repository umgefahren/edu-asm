use crate::{token::PureToken, Parsable, SelfParsed, Span};
use lazy_static::lazy_static;
use regex::Regex;

const LABEL_EXP: &str = "([A-Za-z]+):";

lazy_static! {
    static ref LABEL_RE: Regex = Regex::new(LABEL_EXP).unwrap();
}

#[cfg(test)]
mod label_exp_test {
    use crate::label::LABEL_RE;

    #[test]
    fn valid_labels() {
        const LABELS: [&str; 3] = ["lABEL:", "llllll:", "LABEL:"];

        LABELS.iter().for_each(|e| {
            assert!(LABEL_RE.is_match(*e));
        });
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelToken {
    pub content: String,
}

impl From<LabelToken> for PureToken {
    #[inline]
    fn from(lt: LabelToken) -> Self {
        Self::Label(lt)
    }
}

impl Parsable for LabelToken {
    fn parse(inp: &str) -> Vec<crate::SelfParsed<Self>> {
        LABEL_RE
            .captures_iter(inp)
            .map(|e| {
                let mut group_iter = e.iter();
                let whole_match = group_iter.next().unwrap().unwrap();
                let start = whole_match.start();
                let end = whole_match.end();
                let span = Span { start, end };
                let content = group_iter.next().unwrap().unwrap().as_str().to_string();
                let token = LabelToken { content };
                SelfParsed { inner: token, span }
            })
            .collect()
    }
}
