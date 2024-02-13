use crate::syntax::{expression::SyntaxTreeExpression, lexer::Token};

#[derive(Debug)]
pub enum BinaryOperatorKind {
  Plus,
  Minus,
  Multiply,
  Divide
}
pub struct BinaryOperator {
  kind: BinaryOperatorKind,
  token: Token
}

impl BinaryOperator {
  pub fn new(kind: BinaryOperatorKind, token: Token) -> Self {
    BinaryOperator { kind, token}
  }

  pub fn precedence(&self) -> u8 {
    return match self.kind {
      BinaryOperatorKind::Plus => 1,
      BinaryOperatorKind::Minus => 1,
      BinaryOperatorKind::Multiply => 2,
      BinaryOperatorKind::Divide => 2 
    };
  }
}
pub struct BinaryExpression {
  left: Box<SyntaxTreeExpression>,
  operator: BinaryOperator,
  right: Box<SyntaxTreeExpression>  
}