mod x86;

use crate::x86::*;
const TEST1: &str = "if 1 { print(1 + 2) }";
fn main() {
    let tokens = lex(TEST1);
    let program = Parser::parse(tokens);
    let mut compiler = Compiler::default();
    println!("{}", compiler.compile(program));
}
