// Lexical Analyser for Unilang

use std::fmt::{Display, Formatter, write};

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
  Eof,
  // Add more token types to assess in the parser
  // Keywords
  Identifier,
  Equal,
  Let,

}

impl Display for TokenType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenType::Number(_) => write!(f, "Number"),
      TokenType::Plus => write!(f, "+"),
      TokenType::Minus => write!(f, "-"),
      TokenType::Asterisk => write!(f, "*"),
      TokenType::ForwardSlash => write!(f, "/"),
      TokenType::BackSlash => write!(f, "\\"),
      TokenType::LeftParenthesis => write!(f, "("),
      TokenType::RightParenthesis => write!(f, ")"),
      TokenType::BackSlash => write!(f, "\\"),
      TokenType::BadChar => write!(f, "Bad"),
      TokenType::WhiteSpace => write!(f, "Whitespace"),
      TokenType::Eof => write!(f, "EOF"),
      TokenType::Identifier => write!(f, "Identifier"),
      TokenType::Equal => write!(f, "="),
      TokenType::Let => write!(f, "Let")
    }
  }
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
  pub(crate) span: TextSpan
}

pub struct Lexer<'a> {
  input: &'a str,
  current_position: usize
}

// Interfaces End Here

impl TextSpan {
  pub fn new(start: usize, end: usize, literal: String) -> Self {
    return Self { start, end, literal };
  }

  pub fn length(&self) -> usize {
    return self.end - self.start;
  }
}

impl Token {
  pub fn new(kind: TokenType, span: TextSpan) -> Self {
    return Self { kind, span };
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

    let c = self.current();
    return c.map(|c| {
      let start = self.current_position;
      // println!("Start: {}", start);
      let mut kind = TokenType::BadChar;
      
      if Self::is_number_start(&c) {
        // println!("Consuming Number: {}", c);
        let number: i64 = self.consume_number();
        kind = TokenType::Number(number);
      }
      else if Self::is_whitespace(&c){
        // println!("Consuming Whitespace");
        self.consume();
        kind = TokenType::WhiteSpace
      }
      else if Self::is_identifier_start(&c) {
        let identifier = self.consume_identifier();
        kind = match identifier.as_str() {
          "let" => TokenType::Let,
          _ => TokenType::Identifier
        }
      }
      else {
        kind = self.consume_punctuation();
      }

      let end = self.current_position;
      // Debug Logs
      // println!("End: {}", end);      
      let literal = self.input[start..end].to_string();
      let span = TextSpan::new(start, end, literal);
      return Token::new(kind, span);
    })
    
  }

  fn is_number_start(c: &char) -> bool {
    return c.is_digit(10)
  }

  fn is_identifier_start(c: &char) -> bool {
    return c.is_alphabetic();
  }

  fn is_whitespace(c: &char) -> bool {
    return c.is_whitespace()
  }

  fn consume_punctuation(&mut self) -> TokenType {
    let c = self.consume().unwrap(); 
    // Debug Logs     
    // println!("Consuming Punctuation: {:?}", c);

    return match c {
      '+' => TokenType::Plus,
      '-' => TokenType::Minus,
      '*' => TokenType::Asterisk,
      '/' => TokenType::ForwardSlash,
      '(' => TokenType::LeftParenthesis,
      ')' => TokenType::RightParenthesis,
      '=' => TokenType::Equal,
      '\\' => TokenType::BackSlash,
      _ => TokenType::BadChar
    };
  }

  fn consume_identifier(&mut self) -> String {
    let mut identifier = String::new();
    while let Some(c) = self.current() {
      if Self::is_identifier_start(&c) {
        self.consume().unwrap();
        identifier.push(c)
      }
      else {
        break;
      }
    }
    return identifier;
  }

  fn current(&self) -> Option<char> {
    return self.input.chars().nth(self.current_position);
  }

  fn consume(&mut self) -> Option<char> {
    if self.current_position >= self.input.len() {
       return None;
    }
    let c = self.current();
    self.current_position += 1;    
    return c;
  }

  fn consume_number(&mut self) -> i64 {
    let mut number: i64 = 0;
    while let Some(c) = self.current() {
      if c.is_digit(10) {
        self.consume().unwrap();
        number = number * 10 + c.to_digit(10).unwrap() as i64;
      }
      else { 
        break;
      }
    }
    return number;
  }
}