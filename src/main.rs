mod x86;

use crate::x86::*;
const TEST1: &str = "let x = 1 + 1; print(x)";
fn main() {
    let tokens = lex(TEST1);
    let program = Parser::parse(tokens);
    let mut compiler = Compiler::default();
    println!("{}", compiler.compile(program));
}
