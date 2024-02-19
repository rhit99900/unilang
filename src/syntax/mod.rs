use self::{
  expression::SyntaxTreeExpression,  
  printer::SyntaxTreePrinter, 
  statement::SyntaxTreeStatement, 
  types::{
    _binary::BinaryOperator, 
    _number::NumberExpression
  }, 
  visitor::SyntaxTreeVisitor
};

pub mod lexer;
pub mod parser;
pub mod evaluator;
pub mod visitor;
pub mod statement;
pub mod expression;
pub mod types;
pub mod printer;
pub mod test;

// Abstract Syntax Tree Module
pub struct SyntaxTree {
  pub statements: Vec<SyntaxTreeStatement>
}

impl SyntaxTree {
  pub fn new() -> Self {
    return Self { statements: Vec::new() }
  }

  pub fn add_statement(&mut self, statement: SyntaxTreeStatement) {
    self.statements.push(statement);
  }

  pub fn visit(&self, visitor: &mut dyn SyntaxTreeVisitor) {
    for statement in &self.statements {
      visitor.visit_statement(statement);
    }
  }
  
  pub fn visualise(&self) -> () {
    let mut printer = SyntaxTreePrinter::new();
    self.visit(&mut printer);
    println!("{}", printer.result)
  }
}