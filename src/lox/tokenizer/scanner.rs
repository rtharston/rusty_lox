use super::token::{Token, token_type::Type};

pub(crate) struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    chars: std::str::Chars<'a>,
    line: usize
}

impl<'a> Scanner<'a> {
    pub(crate) fn scan_tokens(source: &'a str) -> Vec<Token> {
        let mut scanner = Scanner { source, tokens: Vec::new(), start: 0, current: 0, chars: source.chars(), line: 1 };

        // scanner.scan_token() over and over
        while let Some(token_type) = scanner.scan_token() {
            scanner.add_token(token_type);
            scanner.start = scanner.current;
        }

        let eof: Token = Token { r#type: Type::Eof, value: "" };
        scanner.tokens.push(eof);
        
        scanner.tokens
    }

    fn scan_token(&mut self) -> Option<Type> {;
        use super::token::token_type::SingleCharacter::*;

        match self.advance() {
            Some(c) => match c {
                '(' => Some(Type::SingleCharacter(LeftParen)),
                ')' => Some(Type::SingleCharacter(RightParen)),
                '{' => Some(Type::SingleCharacter(LeftBrace)),
                '}' => Some(Type::SingleCharacter(RightBrace)),
                ',' => Some(Type::SingleCharacter(Comma)),
                '.' => Some(Type::SingleCharacter(Dot)),
                '-' => Some(Type::SingleCharacter(Minus)),
                '+' => Some(Type::SingleCharacter(Plus)),
                ';' => Some(Type::SingleCharacter(Semicolon)),
                '*' => Some(Type::SingleCharacter(Star)),
                _ => return None // TODO: This should move on, maybe log an error, and not stop the scanner
            },
            None => return None
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn add_token(&mut self, r#type: Type) {
        self.tokens.push(Token { r#type, value: &self.source[self.start..self.current] })
    }
}