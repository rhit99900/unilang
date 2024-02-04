// Lexical Analyser for Unilang

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
  Number(i64),
  Plus,
  Minus,
  Asterisk,
  ForwardSlash,
  BackSlash,
  LeftParenthesis,
  RightParenthesis,
  BadChar,
  WhiteSpace,
  Eof
  // Add more token types to assess in the parser
}

// Interfaces Start Here
#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
  pub(crate) start: usize,
  pub(crate) end: usize,
  pub(crate) literal: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
  pub(crate) kind: TokenType,
  pub(super) span: TextSpan
}

pub struct Lexer<'a> {
  input: &'a str,
  current_position: usize
}

// Interfaces End Here

impl TextSpan {
  pub fn new(start: usize, end: usize, literal: String) -> Self {
    Self { start, end, literal}
  }

  pub fn length(&self) -> usize {
    self.end - self.start
  }
}

impl Token {
  pub fn new(kind: TokenType, span: TextSpan) -> Self {
    Self { kind, span }
  }
}

impl <'a> Lexer<'a> {

  pub fn new(input: &'a str) -> Self {
    Self { input, current_position: 0 }
  }

  pub fn next_token(&mut self) -> Option<Token> {
    // Check if the current character is the end of file
    if self.current_position == self.input.len() {
      let _eof: char = '\0';
      self.current_position += 1;
      return Some(Token::new(
        TokenType::Eof,
        TextSpan::new(0, 0, _eof.to_string())
      ));
    }

    let c = self.current_character();
    println!("Current Character: {:?}, Position: {:?}", c.unwrap(), self.current_position);
    return c.map(|c| {
      let start = self.current_position;
      let mut kind = TokenType::BadChar;
      
      if Self::is_number_start(&c) {
        let number: i64 = self.consume_number();
        kind = TokenType::Number(number);
      }
      else if Self::is_whitespace(&c){
        self.consume_character();
        kind = TokenType::WhiteSpace
      }
      else {
        self.consume_punctuation();
      }

      let end = self.current_position;
      let literal = &self.input[start..end];
      let span = TextSpan::new(start, end, literal.to_string());
      return Token::new(kind, span);
    })
    
  }

  fn is_number_start(c: &char) -> bool {
    return c.is_digit(10)
  }

  fn is_whitespace(c: &char) -> bool {
    return c.is_whitespace()
  }

  fn consume_punctuation(&mut self) -> TokenType {
    let c = self.consume_character().unwrap(); 
    return match c {
      '+' => TokenType::Plus,
      '-' => TokenType::Minus,
      '*' => TokenType::Asterisk,
      '/' => TokenType::ForwardSlash,
      '(' => TokenType::LeftParenthesis,
      ')' => TokenType::RightParenthesis,
      '\\' => TokenType::BackSlash,
      _ => TokenType::BadChar
    };
  }

  fn current_character(&self) -> Option<char> {
    return self.input.chars().nth(self.current_position);
  }

  fn consume_character(&mut self) -> Option<char> {
    if self.current_position >= self.input.len() {
       return None;
    }
    let c = self.current_character();
    self.current_position += 1;    
    return c;
  }

  fn consume_number(&mut self) -> i64 {
    let mut number: i64 = 0;
    while let Some(c) = self.consume_character() {
      println!("{:?} Is Digit: {}", c, c.is_digit(10));
      if c.is_digit(10) {
        // TODO: This needs Fixing!!
        self.consume_character().unwrap();
        number = number * 10 + c.to_digit(10).unwrap() as i64
      }
      else { 
        break;
      }
    }
    return number;
  }
}