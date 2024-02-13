use termion::color::{self, Fg, Reset};

use super::{
  expression::SyntaxTreeExpression, 
  lexer::TextSpan, 
  statement::SyntaxTreeStatement, 
  types::{
    _binary::BinaryExpression, 
    _let::LetStatement, 
    _number::NumberExpression, 
    _parenthesis::ParenthesisExpression, 
    _variable::VariableExpression
  },
  visitor::SyntaxTreeVisitor
};

const INDENT_LEVEL:usize = 2;
pub struct SyntaxTreePrinter {
  pub indent: usize,
  pub(crate) result: String
}

impl SyntaxTreePrinter {
  const NUMBER_COLOR: color::Cyan = color::Cyan;
  const TEXT_COLOR: color::LightWhite = color::LightWhite;
  const KEYWORD_COLOR: color::Magenta = color::Magenta;
  const VARIABLLE_COLOR: color::Green = color::Green;

  pub fn new() -> Self {
    return Self { indent: 0, result: String::new() };
  }

  fn add_whitespace(&mut self) {
    self.result.push_str(" ");
  }

  fn add_new_line(&mut self) {
    self.result.push_str("
    ");
  }  

  fn print_with_indent(&mut self, text: &str) {
    println!("{}{}", " ".repeat(self.indent), text);
  }
}

impl SyntaxTreeVisitor for SyntaxTreePrinter {

  fn visit_let_statement(&mut self, let_statement: &LetStatement) {
      self.result.push_str(&format!("{}let", Self::KEYWORD_COLOR.fg_str()));
      self.add_whitespace();
      self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), let_statement.identifier.span.literal, ));
      self.add_whitespace();
      self.result.push_str(&format!("{}=", Self::TEXT_COLOR.fg_str(),));
      self.add_whitespace();
      self.visit_expression(&let_statement.initialiser);
  }

  fn visit_statement(&mut self, statement: &SyntaxTreeStatement) {
    Self::do_visit_statement(self, statement);
    self.result.push_str(&format!("{}\n", Fg(Reset),));
  }

  fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) {
    self.result.push_str(&format!("{}{}", Self::VARIABLLE_COLOR.fg_str(), variable_expression.identifier.span.literal, ));
  }

  fn visit_error(&mut self, span: &TextSpan) {
    self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), span.literal,));
  }

  fn visit_expression(&mut self, expression: &SyntaxTreeExpression) {
    self.print_with_indent("Expression:");
    self.indent += INDENT_LEVEL;
    SyntaxTreeVisitor::do_visit_expression(self, expression);
    self.indent -= INDENT_LEVEL;
  }

  fn visit_number(&mut self, number: &NumberExpression) {
    self.print_with_indent(&format!("Number: {}", number.number));
  }

  fn visit_binary_expression(&mut self, binary_expression: &BinaryExpression) {
    self.print_with_indent("Binary Expression:");
    self.indent += INDENT_LEVEL;
    self.print_with_indent(&format!("Operator: {:?}", binary_expression.operator.kind));
    self.visit_expression(&binary_expression.left);
    self.visit_expression(&binary_expression.right);
    self.indent -= INDENT_LEVEL;
  }

  fn visit_parenthesised_expression(&mut self, parenthesised_expression: &ParenthesisExpression) {
    self.print_with_indent("Parenthesised Expression:");
    self.indent += INDENT_LEVEL;
    SyntaxTreeVisitor::visit_expression(self, &parenthesised_expression.expression);
    self.indent -= INDENT_LEVEL;
  }
}