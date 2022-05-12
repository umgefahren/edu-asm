use crate::{
    comment::CommentToken, label::LabelToken, label_ref::LabelRefToken, literal::LiteralToken,
    operator::OperatorToken, register::RegisterToken, Parsable, SelfParsed, Span,
};

#[non_exhaustive]
#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PureToken {
    Label(LabelToken),
    LabelRef(LabelRefToken),
    Operator(OperatorToken),
    Literal(LiteralToken),
    Register(RegisterToken),
    Comment(CommentToken),
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositionedToken {
    pub count: usize,
    pub token: PureToken,
    pub span: Span,
}

impl PositionedToken {
    pub fn new<T: Parsable>(inp: SelfParsed<T>) -> Self {
        Self {
            token: inp.inner.into(),
            count: 0,
            span: inp.span,
        }
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenLine {
    pub content: Vec<PositionedToken>,
    pub comment: Option<CommentToken>,
    pub number: usize,
}
