use super::{
  expression::SyntaxTreeExpression, 
  lexer::Token, 
  types::_let::LetStatement
};

pub enum SyntaxTreeStatementKind {
  Expression(SyntaxTreeExpression),
  Let(LetStatement)
}

pub struct SyntaxTreeStatement {
  pub kind: SyntaxTreeStatementKind
}

impl SyntaxTreeStatement {
  pub fn new(kind: SyntaxTreeStatementKind) -> Self {
    return SyntaxTreeStatement { kind };
  }

  pub fn expression(expr: SyntaxTreeExpression) -> Self {
    return SyntaxTreeStatement::new(SyntaxTreeStatementKind::Expression(expr));
  }

  pub fn let_statement(identifier: Token, initialiser: SyntaxTreeExpression) -> Self {    
    return SyntaxTreeStatement::new(SyntaxTreeStatementKind::Let(LetStatement { identifier, initialiser }));
  }
}