#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    LParen,
    RParen,
    EOF,
}

pub struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.src.chars().nth(self.pos)?;
        self.pos += 1;
        Some(ch)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut out = vec![];

        while let Some(c) = self.next_char() {
            match c {
                '+' => out.push(Token::Plus),
                '-' => out.push(Token::Minus),
                '(' => out.push(Token::LParen),
                ')' => out.push(Token::RParen),
                d if d.is_ascii_digit() => {
                    let mut num = d.to_string();
                    while let Some(nc) = self.src.chars().nth(self.pos) {
                        if nc.is_ascii_digit() {
                            num.push(nc);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    out.push(Token::Number(num.parse().unwrap()));
                }
                _ if c.is_whitespace() => {},
                _ => panic!("Unknown char {}", c),
            }
        }

        out.push(Token::EOF);
        out
    }
}
