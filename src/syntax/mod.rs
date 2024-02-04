use self::lexer::Token;

pub mod lexer;
pub mod parser;
pub mod evaluator;

const INDENT_LEVEL:usize = 2;
// Abstract Syntax Tree Module
pub struct SyntaxTree {
  pub statements: Vec<SyntaxTreeStatement>
}

pub trait SyntaxTreeVisitor {
  fn do_visit_statement(&mut self, statement: &SyntaxTreeStatement) {
    match &statement.kind {
      SyntaxTreeStatementKind::Expression(expr) => {
        self.visit_expression(expr);
      }
    }  
  }

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
    }
  }

  fn visit_expression(&mut self, expression: &SyntaxTreeExpression) {
    self.do_visit_expression(expression);
  }

  fn visit_number(&mut self, number: &NumberExpression);

  fn visit_binary_expression(&mut self, binary_expression: &BinaryExpression) {
    self.visit_expression(&binary_expression.left);
    self.visit_expression(&binary_expression.right);
  }

  fn visit_parenthesised_expression(&mut self, parenthesised_expression: &ParenthesisExpression) {
    self.visit_expression(&parenthesised_expression.expression);
  }
}

pub struct SyntaxTreePrinter {
  indent: usize
}


pub enum SyntaxTreeExpressionKind {
  Number(NumberExpression),
  Binary(BinaryExpression),
  Parenthesised(ParenthesisExpression)
}

pub struct SyntaxTreeExpression {
  kind: SyntaxTreeExpressionKind
}

pub struct ParenthesisExpression {
  expression: Box<SyntaxTreeExpression>
}
pub struct NumberExpression {
  number: i64
}

#[derive(Debug)]
pub enum BinaryOperatorKind {
  Plus,
  Minus,
  Multiply,
  Divide
}
pub struct BinaryOperator {
  kind: BinaryOperatorKind,
  token: Token
}

impl BinaryOperator {
  pub fn new(kind: BinaryOperatorKind, token: Token) -> Self {
    BinaryOperator { kind, token}
  }

  pub fn precedence(&self) -> u8 {
    return match self.kind {
      BinaryOperatorKind::Plus => 1,
      BinaryOperatorKind::Minus => 1,
      BinaryOperatorKind::Multiply => 2,
      BinaryOperatorKind::Divide => 2 
    };
  }
}
pub struct BinaryExpression {
  left: Box<SyntaxTreeExpression>,
  operator: BinaryOperator,
  right: Box<SyntaxTreeExpression>  
}

pub enum SyntaxTreeStatementKind {
  Expression(SyntaxTreeExpression)
}

pub struct SyntaxTreeStatement {
  kind: SyntaxTreeStatementKind
}

impl SyntaxTreeExpression {
  pub fn new(kind: SyntaxTreeExpressionKind) -> Self {
    SyntaxTreeExpression { kind }
  }

  pub fn number(number: i64) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Number(NumberExpression { number }));
  }

  pub fn binary(operator: BinaryOperator, left: SyntaxTreeExpression, right: SyntaxTreeExpression) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Binary(BinaryExpression{ left: Box::new(left), operator, right: Box::new(right)}));
  }

  pub fn parenthsised(expression: SyntaxTreeExpression) -> Self {
    return SyntaxTreeExpression::new(SyntaxTreeExpressionKind::Parenthesised(ParenthesisExpression {expression: Box::new(expression)}));
  }
  
}

impl SyntaxTreeStatement {
  pub fn new(kind: SyntaxTreeStatementKind) -> Self {
    return SyntaxTreeStatement { kind };
  }

  pub fn expression(expr: SyntaxTreeExpression) -> Self {
    return SyntaxTreeStatement::new(SyntaxTreeStatementKind::Expression(expr));
  }
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
    let mut printer = SyntaxTreePrinter { indent: 0 };
    self.visit(&mut printer);
  }
}

impl SyntaxTreePrinter {
  fn print_with_indent(&mut self, text: &str) {
    println!("{}{}", " ".repeat(self.indent), text);
  }
}

impl SyntaxTreeVisitor for SyntaxTreePrinter {
  fn visit_statement(&mut self, statement: &SyntaxTreeStatement) {
    self.print_with_indent("Statement:");
    self.indent += INDENT_LEVEL;
    SyntaxTreeVisitor::do_visit_statement(self, statement);
    self.indent -= INDENT_LEVEL;
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