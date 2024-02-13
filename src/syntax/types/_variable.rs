use crate::syntax::lexer::Token;

pub struct VariableExpression {
  pub identifier: Token
}

impl VariableExpression {
  pub fn identifier(&self) -> &str {
    return &self.identifier.span.literal;
  }
}
