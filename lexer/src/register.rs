use crate::{token::PureToken, Parsable, SelfParsed, Span};
use lazy_static::lazy_static;
use regex::Regex;

const REGISTER_EXP: &str = "\\$([G|S|R|I|Z]_([0-7]|B|E))?";

lazy_static! {
    static ref REGISTER_RE: Regex = Regex::new(REGISTER_EXP).unwrap();
}

#[cfg(test)]
mod register_exp_test {
    use crate::register::REGISTER_RE;

    #[test]
    fn valid_registers() {
        const GENERAL_PURPOSE: [&str; 8] = [
            "$G_0", "$G_1", "$G_2", "$G_3", "$G_4", "$G_5", "$G_6", "$G_7",
        ];
        const STACK: [&str; 2] = ["$S_B", "$S_E"];
        const MISC: [&str; 3] = ["$R", "$I", "$Z"];

        GENERAL_PURPOSE
            .iter()
            .chain(STACK.iter())
            .chain(MISC.iter())
            .for_each(|e| {
                assert!(REGISTER_RE.is_match(*e));
            });
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegisterToken {
    pub content: String,
}

impl From<RegisterToken> for PureToken {
    fn from(rt: RegisterToken) -> Self {
        Self::Register(rt)
    }
}

impl Parsable for RegisterToken {
    fn parse(inp: &str) -> Vec<SelfParsed<Self>> {
        let ret = REGISTER_RE.captures_iter(inp).map(|e| {
            let mut group_iter = e.iter();
            let whole_match = group_iter.next().unwrap().unwrap();
            let start = whole_match.start();
            let end = whole_match.end();
            let span = Span { start, end };
            let content = group_iter.next().unwrap().unwrap().as_str().to_string();
            let token = RegisterToken { content };
            SelfParsed { inner: token, span }
        });

        ret.collect()
    }
}
