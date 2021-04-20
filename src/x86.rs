use std::collections::HashMap;
use std::iter::repeat_with;
use std::iter::Peekable;
use std::vec::IntoIter;

const PRELUDE: &str = r#"
section	.text
  global _start
"#;

#[derive(Default)]
pub struct Assembler {
    bss: Vec<String>,
    code: Vec<String>,
    data: Vec<String>,
    pub variables: HashMap<String, String>,
}

impl<'a> Assembler {
    /// Finalize codegen. Put things together.
    pub fn finalize(&self) -> String {
        format!(
            "{}\n{}\nsection .data\n{}\nsegment .bss\n{}",
            PRELUDE,
            self.code.join("\n"),
            self.data.join("\n"),
            self.bss.join("\n")
        )
    }

    /// Declare a new byte variable in the BSS section
    pub fn define_byte(&mut self, name: &'a str) {
        let decl = format!("  {} resb 1", name);
        self.bss.push(decl);
    }

    pub fn declare_byte(&mut self, name: &'a str, value: &'a str) {
        let decl = format!("  {} db {}", name, value);
        self.data.push(decl);
    }

    /// Start a new label
    pub fn label(&mut self, name: &'a str) {
        let label = format!("{}:", name);
        self.code.push(label);
    }

    /// Push a new instruction. Goes under the current label.
    pub fn inst(&mut self, inst: &'a str) {
        self.code.push(format!("  {}", inst));
    }

    pub fn declare_comptime(&mut self, name: &'a str, corr: &'a str) {
        self.variables.insert(name.to_string(), corr.to_string());
    }
}

#[derive(Default)]
pub struct Compiler {
    asm: Assembler,
}

/// Arithmetic operations defined in the AST
#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
}

/// Language literals defined in the AST
#[derive(Debug)]
pub enum Literal {
    Int(i64),
}

/// In-built functions in the language.
#[derive(Debug)]
pub enum InBuiltFn {
    Print(Box<Node>),
}

/// AST Nodes accepted by the compiler
#[derive(Debug)]
pub enum Node {
    Infix {
        lhs: Box<Node>,
        rhs: Box<Node>,
        op: Op,
    },
    Literal(Literal),
    Fn(InBuiltFn),
    Ident(String),
    Assign(String, Box<Node>),
}

pub type Program = Vec<Node>;

impl<'a> Compiler {
    pub fn compile(&mut self, program: Vec<Node>) -> String {
        self.asm.label("_start");

        for node in program {
            self.compile_node(node);
        }

        // sys_exit
        self.asm.inst("mov eax, 1");
        self.asm.inst("int 0x80");

        self.asm.finalize()
    }

    fn compile_node(&mut self, node: Node) -> String {
        let r: String = repeat_with(fastrand::alphanumeric).take(10).collect();
        match node {
            Node::Assign(name, value) => {
                let value = self.compile_node(*value);
                if value.starts_with("'") {
                    self.asm.declare_byte(&name, &value);
                } else {
                    self.asm.declare_comptime(&name, &value);
                }
            }
            Node::Infix { lhs, rhs, op } => {
                let lhs = self.compile_node(*lhs);
                let rhs = self.compile_node(*rhs);

                // Declare literals
                match op {
                    Op::Add | Op::Sub => {
                        self.asm.inst(&format!("mov ecx, {}", lhs));
                        self.asm.inst("sub ecx, '0'");
                        self.asm.inst(&format!("mov edx, {}", rhs));
                        self.asm.inst("sub edx, '0'");
                    }
                }

                // Perform operation
                match op {
                    Op::Add => {
                        self.asm.inst("mov eax, ecx");
                        self.asm.inst("add eax, edx");
                        self.asm.inst("add eax, '0'");
                        self.asm.inst(&format!("mov [{}], eax", r));
                    }
                    Op::Sub => {
                        self.asm.inst("mov eax, ecx");
                        self.asm.inst("sub eax, edx");
                        self.asm.inst("add eax, '0'");
                        self.asm.inst(&format!("mov [{}], eax", r));
                    }
                }
            }
            Node::Literal(lit) => match lit {
                Literal::Int(int) => {
                    return format!("'{}'", int);
                }
            },
            Node::Ident(idt) => {
                return if let Some(holding) = self.asm.variables.get(&idt) {
                    holding.to_string()
                } else {
                    idt
                }
            }
            Node::Fn(inbuilt) => match inbuilt {
                InBuiltFn::Print(node) => {
                    let value = self.compile_node(*node);
                    self.asm.inst(&format!("mov ecx, {}", value));
                    // self.asm.inst("sub ecx, '0'");
                    self.asm.inst("mov edx, 2");
                    self.asm.inst("mov ebx, 1");
                    self.asm.inst("mov eax, 4");
                    self.asm.inst("int 0x80");
                }
            },
        }
        self.asm.define_byte(&r);
        r
    }
}

/// Lexical tokens consumed by the parser.
#[derive(Debug)]
pub enum Token {
    Let,
    Literal(Literal),
    Ident(String),
    Plus,
    LParen,
    RParen,
    Equal,
}

///
fn ident(candidate: String) -> Token {
    match candidate.as_ref() {
        "let" => Token::Let,
        _ => Token::Ident(candidate),
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut input = input.as_bytes().into_iter().peekable();
    'o: loop {
        let ch = input.next();
        if ch.is_none() {
            break 'o;
        }
        let ch = ch.unwrap();
        let token = match ch {
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'=' => Token::Equal,
            b'0'..=b'9' => {
                let mut num = String::new();
                num.push(*ch as char);
                'i: loop {
                    let next_ch = input.peek();
                    match next_ch {
                        Some(b'0'..=b'9') => {
                            num.push(**next_ch.unwrap() as char);
                            input.next();
                        }
                        _ => break 'i Token::Literal(Literal::Int(num.parse::<i64>().unwrap())),
                    }
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let mut idt = String::new();
                idt.push(*ch as char);
                'e: loop {
                    let next_ch = input.peek();
                    match next_ch {
                        Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => {
                            idt.push(**next_ch.unwrap() as char);
                            input.next();
                        }
                        _ => {
                            break 'e ident(idt);
                        }
                    }
                }
            }
            _ => continue 'o,
        };
        tokens.push(token);
    }

    tokens
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Program {
        let mut program: Program = vec![];
        let mut tokens = tokens.into_iter().peekable();
        loop {
            let node = Self::parse_token(&mut tokens);

            match node {
                Some(node) => program.push(node),
                None => break,
            }
        }

        program
    }

    fn parse_token(tokens: &mut Peekable<IntoIter<Token>>) -> Option<Node> {
        let token = tokens.next();
        if token.is_none() {
            return None;
        };

        let token = token.unwrap();

        let lhs = match token {
            Token::Literal(literal) => Node::Literal(literal),
            Token::Let => {
                match tokens.peek() {
                    Some(Token::Ident(name)) => {
                        let name = name.to_string();
                        tokens.next();
                        tokens.next(); // Skip `=`
                        let value = Self::parse_token(tokens).unwrap();
                        Node::Assign(name, Box::new(value))
                    }
                    _ => panic!("Expected Ident after `let`"),
                }
            }
            Token::Ident(ident) => match tokens.peek() {
                Some(Token::LParen) => {
                    tokens.next();
                    let param = Self::parse_token(tokens).unwrap();
                    tokens.next();
                    Node::Fn(InBuiltFn::Print(Box::new(param)))
                }
                _ => Node::Ident(ident),
            },
            _ => panic!("TODO"),
        };

        let node = match tokens.peek() {
            Some(Token::Plus) => {
                tokens.next();
                let rhs = Self::parse_token(tokens);

                if rhs.is_none() {
                    return None;
                } else {
                    Node::Infix {
                        op: Op::Add,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs.unwrap()),
                    }
                }
            }
            _ => lhs,
        };

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::x86::*;

    const TEST1: &str = "1 + 1";
    #[test]
    fn test_parse() {
        let tokens = lex(TEST1);
        let program = Parser::parse(tokens);
        let mut compiler = Compiler::default();

        compiler.compile(program);
    }
}
