use crate::syntax::{BinaryExpression, BinaryOperatorKind, NumberExpression, SyntaxTreeVisitor};

pub struct Evaluator {
  pub last_value: Option<i64>
}

impl Evaluator {
  pub fn new() -> Self {
    Self { last_value: None }
  }
}

impl SyntaxTreeVisitor for Evaluator {
  fn visit_number(&mut self, number: &NumberExpression) {
    self.last_value = Some(number.number);
  }

  fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
    self.visit_expression(&expr.left);
    let left = self.last_value.unwrap();
    self.visit_expression(&expr.right);
    let right = self.last_value.unwrap();
    self.last_value = Some(match expr.operator.kind {
      BinaryOperatorKind::Plus => left + right,
      BinaryOperatorKind::Minus => left - right,
      BinaryOperatorKind::Multiply => left * right,
      BinaryOperatorKind::Divide => left / right
    });
  }
}