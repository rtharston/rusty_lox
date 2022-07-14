#[path ="scanner.rs"]
mod scanner;

mod token;

use token::Token;

pub struct Tokenizer<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer { tokens: scanner::Scanner::scan_tokens(source) }
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens.as_ref()
    }
}
