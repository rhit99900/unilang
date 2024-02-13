use termion::color::Reset;
use super::expression::SyntaxTreeExpressionKind;
use super::expression::SyntaxTreeExpression;
use super::lexer::TextSpan;
use super::printer::SyntaxTreePrinter;
use super::statement::{SyntaxTreeStatement, SyntaxTreeStatementKind};
use super::types::_binary::BinaryExpression;
use super::types::_number::NumberExpression;
use super::types::_let::LetStatement;
use super::types::_parenthesis::ParenthesisExpression;
use super::types::_variable::VariableExpression;

pub trait SyntaxTreeVisitor {
  fn do_visit_statement(&mut self, statement: &SyntaxTreeStatement) {
    match &statement.kind {
      SyntaxTreeStatementKind::Expression(expr) => {
        self.visit_expression(expr);
      }
      SyntaxTreeStatementKind::Let(expr) => {
        self.visit_let_statement(expr);
      }
    }  
  }

  fn visit_let_statement(&mut self, let_statement: &LetStatement);
  fn visit_statement(&mut self, statement: &SyntaxTreeStatement) {
    self.do_visit_statement(statement)
  }

  fn do_visit_expression(&mut self, expression: &SyntaxTreeExpression) {
    match &expression.kind {
      SyntaxTreeExpressionKind::Number(number) => {
        self.visit_number(number);
      }
      SyntaxTreeExpressionKind::Binary(expr) => {
        self.visit_binary_expression(expr);
      }
      SyntaxTreeExpressionKind::Parenthesised(expr) => {
        self.visit_parenthesised_expression(expr);
      }
      SyntaxTreeExpressionKind::Error(span) => {
        self.visit_error(span);
      }
      SyntaxTreeExpressionKind::Variable(expr) => {
        self.visit_variable_expression(expr);
      }
    }
  }

  fn visit_expression(&mut self, expression: &SyntaxTreeExpression) {
    self.do_visit_expression(expression);
  }

  fn visit_number(&mut self, number: &NumberExpression);

  fn visit_variable_expression(&mut self, variable_expression: &VariableExpression);

  fn visit_error(&mut self, span: &TextSpan);

  fn visit_binary_expression(&mut self, binary_expression: &BinaryExpression) {
    self.visit_expression(&binary_expression.left);
    self.visit_expression(&binary_expression.right);
  }

  fn visit_parenthesised_expression(&mut self, parenthesised_expression: &ParenthesisExpression) {
    self.visit_expression(&parenthesised_expression.expression);
  }   
}