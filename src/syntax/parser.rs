use crate::syntax::{BinaryOperator, BinaryOperatorKind, SyntaxTreeExpression, SyntaxTreeStatement};
use crate::syntax::lexer::{Lexer, Token, TokenType};

pub struct Parser {
  tokens: Vec<Token>,
  current: usize
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens: tokens.iter().filter(
        |token| token.kind != TokenType::WhiteSpace
      ).map(|token| token.clone()).collect(),
      current: 0    
    }
  }

  pub fn next_statement(&mut self) -> Option<SyntaxTreeStatement> {
    return self.parse_statement();
  }

  fn parse_statement(&mut self) -> Option<SyntaxTreeStatement> {
    let token = self.current()?;
    if token.kind == TokenType::Eof {
      return None;
    }
    let expr = self.parse_expression()?;
    return Some(SyntaxTreeStatement::expression(expr));
  }

  fn parse_expression(&mut self) -> Option<SyntaxTreeExpression> {
    return self.parse_binary_expression(0);
  }

  fn parse_binary_expression(&mut self, precedence: u8) -> Option<SyntaxTreeExpression> {
    let mut left = self.parse_primary_expression()?;
    while let Some(operator) = self.parse_binary_operator() {
      self.consume();
      let operator_precedence = operator.precedence();
      if operator_precedence < precedence {
        break;
      }
      let right = self.parse_binary_expression(operator_precedence)?;
      left = SyntaxTreeExpression::binary(operator, left, right);
    }
    return Some(left);
  }

  fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
    let token = self.current()?;
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

  fn parse_primary_expression(&mut self) -> Option<SyntaxTreeExpression> {
    let token = self.consume()?;
    return match token.kind {
      TokenType::Number(number) => {
        Some(SyntaxTreeExpression::number(number))
      }
      TokenType::LeftParenthesis => {
        let expr = self.parse_expression()?;
        let token = self.consume()?;
        if token.kind != TokenType::RightParenthesis {
          panic!("Expected Right Parenthesis")
        }
        Some(
          SyntaxTreeExpression::parenthsised(expr)
        )
      }
      _ => {
        None
      }
    }
  }

  fn peek(&self, offset: isize) -> Option<&Token> {
    return self.tokens.get((self.current as isize + offset) as usize);
  }

  fn current(&self) -> Option<&Token> {
    return self.peek(0);
  }

  fn consume(&mut self) -> Option<&Token> {
    self.current += 1;
    let token = self.peek(-1)?;
    return Some(token);
  }
}