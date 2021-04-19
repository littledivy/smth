mod x86;

use crate::x86::*;
const TEST1: &str = "1 + 2 + 2 + 10000";
fn main() {
    let tokens = lex(TEST1);
    let program = Parser::parse(tokens);
    let mut compiler = Compiler::default();
    println!("{}", compiler.compile(program));
}
