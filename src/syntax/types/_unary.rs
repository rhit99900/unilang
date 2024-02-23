use crate::syntax::{
  expression::SyntaxTreeExpression,
  lexer::Token
};

#[derive(Debug)]
pub enum UnaryOperatorKind {
  Minus,
  BitwiseNot
}
pub struct UnaryOperator {
  pub kind: UnaryOperatorKind,
  pub token: Token
}

impl UnaryOperator {
  pub fn new(kind: UnaryOperatorKind, token: Token) -> Self {
    UnaryOperator { kind, token}
  }  
}
pub struct UnaryExpression {
  pub operator: UnaryOperator,
  pub operand: Box<SyntaxTreeExpression>
}