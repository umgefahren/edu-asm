use std::{fmt::Debug, hash::Hash, ops::Range};

use comment::CommentToken;
use label::LabelToken;
use label_ref::LabelRefToken;
use literal::LiteralToken;
use operator::OperatorToken;
use register::RegisterToken;
use token::{PositionedToken, PureToken, TokenLine};

use rayon::prelude::*;

pub mod comment;
pub mod label;
pub mod label_ref;
pub mod literal;
pub mod operator;
pub mod register;
pub mod token;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelfParsed<T: Clone + Hash + Debug + PartialEq + Eq + PartialOrd + Ord> {
    pub inner: T,
    pub span: Span,
}

pub trait Parsable:
    Sized + Into<PureToken> + Clone + Debug + Hash + Debug + PartialEq + Eq + PartialOrd + Ord
{
    fn parse(inp: &str) -> Vec<SelfParsed<Self>>;
}

pub fn lex(inp: &str) -> Vec<TokenLine> {
    inp.split('\n')
        .collect::<Vec<&str>>()
        .par_iter()
        .enumerate()
        .map(|e| {
            let line_number = e.0;
            let line_str = e.1;
            let comment = CommentToken::parse(line_str).get(0).map(|m| m.clone());
            let rest_str = match comment {
                Some(ref d) => &line_str[..d.span.start],
                None => line_str,
            };

            let labels = LabelToken::parse(rest_str)
                .into_iter()
                .map(PositionedToken::new);
            let label_refs = LabelRefToken::parse(rest_str)
                .into_iter()
                .map(PositionedToken::new);
            let literals = LiteralToken::parse(rest_str)
                .into_iter()
                .map(PositionedToken::new);
            let operators = OperatorToken::parse(rest_str)
                .into_iter()
                .map(PositionedToken::new);
            let registers = RegisterToken::parse(rest_str)
                .into_iter()
                .map(PositionedToken::new);
            let mut all_tokens: Vec<PositionedToken> = labels
                .chain(label_refs)
                .chain(literals)
                .chain(operators)
                .chain(registers)
                .collect();
            all_tokens.sort_by_key(|m| m.span.start);
            all_tokens.iter_mut().enumerate().for_each(|m| {
                let index = m.0;
                m.1.count = index;
            });
            TokenLine {
                content: all_tokens,
                comment: comment.map(|m| m.inner),
                number: line_number,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
