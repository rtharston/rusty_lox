#[allow(dead_code)]
#[derive(Debug)]
pub enum Type {
  SingleCharacter(SingleCharacter),
  OneOrTwoCharacters(OneOrTwoCharacters),
  Literals(Literals),
  Keywords(Keywords),
  Eof
}

#[derive(Debug)]
pub enum SingleCharacter {
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star
}

#[derive(Debug)]
pub enum OneOrTwoCharacters {
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual
}

#[derive(Debug)]
pub enum Literals {
  Identifier, String, Number
}

#[derive(Debug)]
pub enum Keywords {
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While
}