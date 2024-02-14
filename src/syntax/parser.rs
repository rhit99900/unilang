use std::cell::Cell;

use crate::diagnostics::DiagnosticGlossaryCell;

use super::{
  expression::SyntaxTreeExpression, 
  lexer::{Token, TokenType}, 
  statement::SyntaxTreeStatement, types::_binary::{BinaryOperator, BinaryOperatorKind}
};

pub struct Counter {
  value: Cell<usize>
}

impl Counter {
  pub fn new() -> Self {
    return Self { value: Cell::new(0) };
  }

  pub fn increment(&self) {
    let current_value = self.value.get();
    self.value.set(current_value + 1);    
  }

  pub fn get_value(&self) -> usize {
    return self.value.get();
  }
}

pub struct Parser {
  tokens: Vec<Token>,
  current: Counter,
  diagnostics_glossary: DiagnosticGlossaryCell
}

impl Parser {
  pub fn new(
    tokens: Vec<Token>, 
    diagnostics_glossary: DiagnosticGlossaryCell
  ) -> Self {
    Self {
      tokens: tokens.iter().filter(
        |token| token.kind != TokenType::WhiteSpace
      ).map(|token| token.clone()).collect(),
      current: Counter::new(),
      diagnostics_glossary
    }
  }

  pub fn next_statement(&mut self) -> Option<SyntaxTreeStatement> {
    if self.is_at_end() {
      return None;
    }
    return Some(self.parse_statement());
  }

  pub fn is_at_end(&self) -> bool {
    return self.current().kind == TokenType::Eof;
  }

  fn parse_statement(&mut self) -> SyntaxTreeStatement {
    match self.current().kind {
       TokenType::Let => {
        self.parse_let_statement()
       }
       _ => {
        self.parse_expression_statement()
       }
    }    
  }

  fn parse_let_statement(&mut self) -> SyntaxTreeStatement {
    self.consume_and_check(TokenType::Let);
    let identifier = self.consume_and_check(TokenType::Identifier).clone();
    self.consume_and_check(TokenType::Equal);
    let expr = self.parse_expression();
    return SyntaxTreeStatement::let_statement(identifier, expr);    
  }

  fn parse_expression_statement(&mut self) -> SyntaxTreeStatement {
    let expr = self.parse_expression();
    return SyntaxTreeStatement::expression(expr);
  }

  fn parse_expression(&mut self) -> SyntaxTreeExpression {
    return self.parse_binary_expression(0);
  }

  fn parse_binary_expression(&mut self, precedence: u8) -> SyntaxTreeExpression {
    let mut left = self.parse_primary_expression();    
    while let Some(operator) = self.parse_binary_operator() {
      self.consume();
      let operator_precedence = operator.precedence();
      if operator_precedence < precedence {
        break;
      }
      let right = self.parse_binary_expression(operator_precedence);
      left = SyntaxTreeExpression::binary(operator, left, right);
    }
    return left;
  }

  fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
    let token = self.current();
    let kind = match token.kind {
      TokenType::Plus => {
        Some(BinaryOperatorKind::Plus)
      }
      TokenType::Minus => {
        Some(BinaryOperatorKind::Minus)
      }
      TokenType::Asterisk => {
        Some(BinaryOperatorKind::Multiply)
      }
      TokenType::ForwardSlash => {
        Some(BinaryOperatorKind::Divide)
      }
      _ => {
        None
      }
    };
    return kind.map(|kind| BinaryOperator::new(kind, token.clone()));
  }

  fn parse_primary_expression(&mut self) -> SyntaxTreeExpression {
    let token = self.consume();
    return match token.kind {
      TokenType::Number(number) => {
        SyntaxTreeExpression::number(number)
      }
      TokenType::LeftParenthesis => {
        let expr = self.parse_expression();
        let token = self.consume();
        if token.kind != TokenType::RightParenthesis {
          panic!("Expected Right Parenthesis")
        }
        SyntaxTreeExpression::parenthsised(expr)        
      }
      TokenType::Identifier => {
        SyntaxTreeExpression::identifier(token.clone())
      }
      _ => {
        self.diagnostics_glossary.borrow_mut().report_expexted_expression(token);
        SyntaxTreeExpression::error(
          token.span.clone()
        )
      }
    }
  }

  fn peek(&self, offset: isize) -> &Token {
    let mut index = (self.current.get_value() as isize + offset) as usize;
    if index >= self.tokens.len() {
      index = self.tokens.len() - 1;
    }
    return self.tokens.get(index).unwrap();
  }

  fn current(&self) -> &Token {
    return self.peek(0);
  }

  fn consume(&mut self) -> &Token {
    self.current.increment();
    return self.peek(-1);
  }

  fn consume_and_check(&mut self, kind: TokenType) -> &Token {
    let token = self.consume();
    if token.kind != kind {
      self.diagnostics_glossary.borrow_mut().report_unexpected_token(&kind, token);
    }
    return token;
  }
}