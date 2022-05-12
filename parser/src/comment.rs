use lazy_static::lazy_static;
use regex::Regex;

const COMMENT_EXP: &str = "#.*";

lazy_static! {
    static ref COMMENT_RE: Regex = Regex::new(COMMENT_EXP).unwrap();
}

pub fn strip_coment(inp: &str) -> &str {

    if let Some(comment_match) = COMMENT_RE.captures(inp) {
        return &inp[..comment_match.get(0).unwrap().start()]
    }

    &inp[..]
}
