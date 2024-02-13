use super::types::{_binary::BinaryExpression, _number::NumberExpression, _parenthesis::ParenthesisExpression};

pub enum SyntaxTreeExpressionKind {
  Number(NumberExpression),
  Binary(BinaryExpression),
  Parenthesised(ParenthesisExpression)
}

pub struct SyntaxTreeExpression {
  pub kind: SyntaxTreeExpressionKind
}