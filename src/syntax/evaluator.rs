use std::collections::HashMap;

use super::{lexer::TextSpan, types::{_binary::{BinaryExpression, BinaryOperatorKind}, _let::LetStatement, _number::NumberExpression, _variable::VariableExpression}, visitor::SyntaxTreeVisitor};


pub struct Evaluator {
  pub last_value: Option<i64>,
  pub variables: HashMap<String, i64>
}

impl Evaluator {
  pub fn new() -> Self {
    Self { 
      last_value: None, 
      variables: HashMap::new() 
    }
  }
}

impl SyntaxTreeVisitor for Evaluator {
  fn visit_number(&mut self, number: &NumberExpression) {
    self.last_value = Some(number.number);
  }

  fn visit_let_statement(&mut self, let_statement: &LetStatement) {
    self.visit_expression(&let_statement.initialiser);
    self.variables.insert(let_statement.identifier.span.literal.clone(), self.last_value.unwrap());
  }
  
  fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) {
    self.last_value = Some(*self.variables.get(&variable_expression.identifier.span.literal).unwrap());
  }

  fn visit_error(&mut self, span: &TextSpan) {
    todo!()
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