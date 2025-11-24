use crate::tokenizer::Token;
use crate::ast::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn eat(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_factor();

        loop {
            match self.current() {
                Token::Plus => {
                    self.eat();
                    let right = self.parse_factor();
                    node = Expr::Binary {
                        left: Box::new(node),
                        op: "+".to_string(),
                        right: Box::new(right)
                    };
                }
                Token::Minus => {
                    self.eat();
                    let right = self.parse_factor();
                    node = Expr::Binary {
                        left: Box::new(node),
                        op: "-".to_string(),
                        right: Box::new(right)
                    };
                }
                _ => break
            }
        }

        node
    }

    fn parse_factor(&mut self) -> Expr {
        match self.eat() {
            Token::Number(n) => Expr::Number(n),
            Token::LParen => {
                let expr = self.parse_expr();
                if let Token::RParen = self.eat() {
                    expr
                } else {
                    panic!("Missing ')'");
                }
            }
            t => panic!("Unexpected token: {:?}", t),
        }
    }
}
