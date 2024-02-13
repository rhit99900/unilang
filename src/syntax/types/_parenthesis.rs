use crate::syntax::expression::SyntaxTreeExpression;

pub struct ParenthesisExpression {
  pub expression: Box<SyntaxTreeExpression>
}
