use std::collections::HashMap;

use crate::{
  diagnostics::DiagnosticGlossaryCell, 
  syntax::{
    lexer::TextSpan, types::{
      _let::LetStatement, _number::NumberExpression, _variable::VariableExpression
    }, visitor::SyntaxTreeVisitor
  }
};


pub struct SymbolChecker {
  symbols: HashMap<String, ()>,
  diagnostics: DiagnosticGlossaryCell
}

impl SymbolChecker {
  pub fn new(diagnostics: DiagnosticGlossaryCell) -> Self {
    SymbolChecker {
      symbols: HashMap::new(),
      diagnostics
    }
  }
}

impl SyntaxTreeVisitor for SymbolChecker {
  fn visit_let_statement(&mut self, let_statement: &LetStatement) {
    let identifier = let_statement.identifier.span.literal.clone();
    self.visit_expression(&let_statement.initialiser);
    self.symbols.insert(identifier, ());
  } 

  fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) {
    if self.symbols.get(&variable_expression.identifier.span.literal).is_none() {
      let mut diagnostics_binding = self.diagnostics.borrow_mut();
      diagnostics_binding.report_undeclared_variable(&variable_expression.identifier);
    }
  }

  fn visit_number(&mut self, number: &NumberExpression) {
    // TODO Remove Print
    println!("{:?}", number);
    // TODO 
  }

  fn visit_error(&mut self, span: &TextSpan) {
    // TODO Remove Print
    println!("{:?}", span);
    // TODO
  }
}