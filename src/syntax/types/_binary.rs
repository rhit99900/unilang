use crate::syntax::{expression::SyntaxTreeExpression, lexer::Token};

#[derive(Debug)]
pub enum BinaryOperatorKind {
  Plus,
  Minus,
  Multiply,
  Divide,
  Power,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor
}
pub struct BinaryOperator {
  pub kind: BinaryOperatorKind,
  pub token: Token
}

impl BinaryOperator {
  pub fn new(kind: BinaryOperatorKind, token: Token) -> Self {
    BinaryOperator { kind, token}
  }

  pub fn precedence(&self) -> u8 {
    return match self.kind {
      BinaryOperatorKind::Power => 20,      
      BinaryOperatorKind::Multiply => 19,
      BinaryOperatorKind::Divide => 19 ,      
      BinaryOperatorKind::Plus => 18,
      BinaryOperatorKind::Minus => 18,
      BinaryOperatorKind::BitwiseAnd => 17,
      BinaryOperatorKind::BitwiseXor => 16,
      BinaryOperatorKind::BitwiseOr => 15
    };
  }
}
pub struct BinaryExpression {
  pub left: Box<SyntaxTreeExpression>,
  pub operator: BinaryOperator,
  pub right: Box<SyntaxTreeExpression>
}