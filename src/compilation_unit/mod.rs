use std::{cell::RefCell, rc::Rc};

use crate::{
  diagnostics::{
    self,
    printer::DiagnosticPrinter,
    DiagnosticGlossaryCell
  }, 
  symbols, 
  syntax::{
    evaluator::Evaluator,
    lexer::Lexer,
    parser::Parser,
    SyntaxTree
  }, 
  text::{
    self,
    SourceText
  }
};

pub struct CompilationUnit {
  pub st: SyntaxTree,
  pub diagnostic_glossary: DiagnosticGlossaryCell
}

impl CompilationUnit {

  pub fn compile(input: &str) -> CompilationUnit {
    let text = text::SourceText::new(input.to_string());
    let mut lexer = Lexer::new(input); 
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }

    // Printing Tokens tokenised by Lexer;
    // Debug Logs
    for token in &tokens {
    		println!("{:?}", token);
    }

    // Diagnostics
    let _diagnostic: DiagnosticGlossaryCell = Rc::new(RefCell::new(diagnostics::DiagnosticGlossary::new()));	
    // Parsing Tokens	
    let mut syntax_tree: SyntaxTree = SyntaxTree::new();
    let mut parser = Parser::new(
      tokens,
      Rc::clone(&_diagnostic)
    );
    // Parse Statements 
    while let Some(stmt) = parser.next_statement() {
        syntax_tree.add_statement(stmt);
    }		
    syntax_tree.visualise();

    if Self::diagnose(&text, &_diagnostic).is_err() {
      return Self::create_compilation_unit(syntax_tree, _diagnostic);
    }
    let mut symbol_checker = symbols::SymbolChecker::new(Rc::clone(&_diagnostic));
    syntax_tree.visit(&mut symbol_checker);
    
    if Self::diagnose(&text, &_diagnostic).is_err() {
      return Self::create_compilation_unit(syntax_tree, _diagnostic);
    }
    return Self::create_compilation_unit(syntax_tree, _diagnostic);
    
  }

  fn diagnose(text: &SourceText, diagnostic_glossary: &DiagnosticGlossaryCell) -> Result<(),()> {
    let diagnositcs_binding = diagnostic_glossary.borrow();
    if diagnositcs_binding.diagnostics.len() > 0 {
      let diaprinter = DiagnosticPrinter::new(&text, &diagnositcs_binding.diagnostics);
      diaprinter.print();
      return Err(());
    }
    return Ok(());
  }

  fn create_compilation_unit(st: SyntaxTree, diagnostic_glossary: DiagnosticGlossaryCell) -> CompilationUnit {
    CompilationUnit {
      st,
      diagnostic_glossary
    }
  }

  pub fn prerun(&self) {
    if self.diagnostic_glossary.borrow().diagnostics.len() > 0 {
      return;
    }
    self.run();
  }

  pub fn run(&self) {
    let mut evaluate = Evaluator::new();
	  self.st.visit(&mut evaluate);
	  println!("Result: {:?}", evaluate.last_value);
  } 
}