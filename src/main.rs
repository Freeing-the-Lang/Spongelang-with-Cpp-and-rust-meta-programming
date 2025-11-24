mod tokenizer;
mod ast;
mod parser;
mod semantic;
mod backend;

use tokenizer::Tokenizer;

fn main() {
    let input = std::fs::read_to_string("examples/hello.sp")
        .expect("Failed to read file");

    let tokens = Tokenizer::new(&input).tokenize();
    println!("Tokens: {:?}", tokens);
}
