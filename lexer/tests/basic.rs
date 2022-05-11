use edu_wasm_lexer::lex;

const SAMPLE_PROGRAM: &str = include_str!("../../examples/basic.edu");

#[test]
fn basic_test() {
    let lexed = lex(SAMPLE_PROGRAM);
    println!("{:?}", lexed);
}
