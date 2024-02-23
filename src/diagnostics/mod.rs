pub mod printer;
pub mod test;
use std::{cell::RefCell, rc::Rc};

use crate::syntax::lexer::{TextSpan, Token, TokenType};

#[derive(Clone, Debug, Copy)]
pub enum DiagnosticKind {
  Error,
  Warning
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
  pub message: String,
  pub span: TextSpan,
  pub kind: DiagnosticKind
}

pub type DiagnosticGlossaryCell = Rc<RefCell<DiagnosticGlossary>>;

impl Diagnostic {
  pub fn new(message: String, span: TextSpan, kind: DiagnosticKind) -> Self {
    return Diagnostic { message, span, kind };
  }  
}

pub struct DiagnosticGlossary {
  pub diagnostics: Vec<Diagnostic>
}

impl DiagnosticGlossary {
  pub fn new() -> Self {
    DiagnosticGlossary { diagnostics: vec![]}
  }

  pub fn report_error(&mut self, message: String, span: TextSpan) {
    let error = Diagnostic::new(message, span, DiagnosticKind::Error);
    self.diagnostics.push(error);
  }

  pub fn report_warning(&mut self, message: String, span: TextSpan) {
    let warning = Diagnostic::new(message, span, DiagnosticKind::Warning);
    self.diagnostics.push(warning);
  }

  pub fn report_unexpected_token(&mut self, expected: &TokenType, token: &Token) {
    self.report_error(format!("Expected <{}>, found <{}>", expected, token.kind), token.span.clone());
  }

  pub fn report_expexted_expression(&mut self, token: &Token) {
    self.report_error(format!("Expected expression, found <{}>", token.kind), token.span.clone());
  }

  pub fn report_undeclared_variable(&mut self, token: &Token) {
    self.report_error(format!("Undeclared variable '{}'", token.span.literal), token.span.clone());
  }
}