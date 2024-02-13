use crate::syntax::{expression::SyntaxTreeExpression, lexer::Token};

pub struct LetStatement {
  pub identifier: Token,
  pub initialiser: SyntaxTreeExpression
}
