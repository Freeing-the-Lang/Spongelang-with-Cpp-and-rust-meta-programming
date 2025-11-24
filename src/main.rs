mod tokenizer;
mod ast;
mod parser;
mod semantic;
mod backend;

use std::env;
use std::fs;
use backend::asm_codegen::AsmCodegen;

fn main() {
    let path = env::args().nth(1).unwrap_or("examples/hello.sp".to_string());
    let input = fs::read_to_string(&path)
        .expect("Failed to read input file");

    // 1. Tokenize
    let tokens = tokenizer::Tokenizer::new(&input).tokenize();

    // 2. Parse
    let mut parser = parser::Parser::new(tokens);
    let expr = parser.parse_expr();

    // 3. Semantic analysis
    semantic::SemanticAnalyzer::analyze(&expr);

    // 4. Generate ASM
    let asm = AsmCodegen::emit(&expr);
    print!("{}", asm);
}
