use super::{
  lexer::{
    TextSpan,
    Token
  }, 
  types::{
    _binary::{
      BinaryExpression,
      BinaryOperator
    }, 
    _number::NumberExpression, 
    _parenthesis::ParenthesisExpression,
    _unary::UnaryExpression,
    _variable::VariableExpression
  }
};

pub enum SyntaxTreeExpressionKind {
  Number(NumberExpression),
  Binary(BinaryExpression),
  Unary(UnaryExpression),
  Parenthesised(ParenthesisExpression),
  Variable(VariableExpression),
  Error(TextSpan)
}

pub struct SyntaxTreeExpression {
  pub kind: SyntaxTreeExpressionKind
}

impl SyntaxTreeExpression {
  pub fn new(kind: SyntaxTreeExpressionKind) -> Self {
    SyntaxTreeExpression { kind }
  }

  pub fn number(number: i64) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Number(NumberExpression { number }));
  }

  pub fn binary(operator: BinaryOperator, left: SyntaxTreeExpression, right: SyntaxTreeExpression) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Binary(BinaryExpression{ left: Box::new(left), operator, right: Box::new(right)}));
  }

  pub fn parenthsised(expression: SyntaxTreeExpression) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Parenthesised(ParenthesisExpression {expression: Box::new(expression)}));
  }

  pub fn identifier(identifier: Token) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Variable(VariableExpression { identifier }));
  }

  pub fn error(span: TextSpan) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Error(span));
  }
}