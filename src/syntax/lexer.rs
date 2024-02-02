#[derive(Debug,PartialEq,Clone)]
pub enum TokenType {
  Number(i64),
  // Plus,
  // Minus,
  // Asterisk,
  // Slash,
  // LeftParen,
  // RightParen,
  Eof,
  BadChar
  
  // Add more token types to assess in the parser
}

#[derive(Debug,PartialEq,Clone)]
pub struct TextSpan {
  start: usize,
  end: usize,
  literal: String
}

impl TextSpan {
  pub fn new(start: usize, end: usize, literal: String) -> Self {
    Self { start, end, literal}
  }

  pub fn length(&self) -> usize {
    self.end - self.start
  }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Token {
  kind: TokenType,
  span: TextSpan
}

impl Token {
  pub fn new(kind: TokenType, span: TextSpan) -> Self {
    Self { kind, span }
  }
}

pub struct Lexer<'a> {
  input: &'a str,
  current_position: usize
}

impl <'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self { input, current_position: 0 }
  }

  pub fn next_token(&mut self) -> Option<Token> {
    if self.current_position > self.input.len() {
      return None;
    }
    if self.current_position == self.input.len() {
      let _eof: char = '\0';
      self.current_position += 1;
      return Some(Token::new(
        TokenType::Eof,
        TextSpan::new(0, 0, _eof.to_string())
      ));
    }

    let c = self.current_character();
    return c.map(|c| {
      let _start = self.current_position;
        let mut kind = TokenType::BadChar;
      if Self::is_number_start(&c) {
        let number: i64 = self.consume_number();
        kind = TokenType::Number(number);
      }
      else {
        self.consume_character();
      }

      let _end = self.current_position;
      let _literal = self.input[_start.._end].to_string();
      let span = TextSpan::new(_start, _end, _literal);
      return Token::new(kind, span);
    })
    
  }

  fn is_number_start(c: &char) -> bool {
    c.is_digit(10)
  }

  fn current_character(&self) -> Option<char> {
    return self.input.chars().nth(self.current_position);
  }

  fn consume_character(&mut self) -> Option<char> {
    if self.current_position > self.input.len() {
       return None;
    }
    let c = self.current_character();
    self.current_position += 1;    
    return c;
  }

  fn consume_number(&mut self) -> i64 {
    let mut number: i64 = 0;
    while let Some(c) = self.consume_character() {
      if c.is_digit(10) {
        number = number * 10 + c.to_digit(10).unwrap() as i64
      }
      else { 
        break;
      }
    }
    number
  }
}