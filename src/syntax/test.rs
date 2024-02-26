
#[cfg(test)]
mod test {
  use crate::{
    compilation_unit::CompilationUnit, 
    syntax::{
      lexer::TextSpan, types::{
        _binary::BinaryExpression, _let::LetStatement, _number::NumberExpression, _parenthesis::ParenthesisExpression, _unary::UnaryExpression, _variable::VariableExpression
      }, 
      visitor::SyntaxTreeVisitor,
      SyntaxTree
    }
  };

  
  #[derive(Debug, PartialEq, Eq)]
  enum SyntaxTreeTestNode {
    Number(i64),
    Binary,
    Unary,
    Parenthesised,
    LetStatement,
    Variable(String)
  }

  struct SyntatTreeVerifier {
    expected: Vec<SyntaxTreeTestNode>,
    actual: Vec<SyntaxTreeTestNode>
  }

  impl SyntatTreeVerifier {
    pub fn new(input: &str, expected: Vec<SyntaxTreeTestNode>) -> Self {
      let compilation_unit = CompilationUnit::compile(input);
      assert_eq!(compilation_unit.diagnostic_glossary.borrow().diagnostics.len(), 0, "Expected no diagnostics, got {:?} instead", compilation_unit.diagnostic_glossary.borrow().diagnostics);
      let mut verifier = SyntatTreeVerifier { expected, actual: Vec::new() };
      verifier.flatten_syntax_tree(&compilation_unit.st);
      return verifier;
    }

    fn flatten_syntax_tree(&mut self, syntax_tree: &SyntaxTree) {
      self.actual.clear();
      syntax_tree.visit(&mut *self);
    }  

    pub fn verify(&self) {

      assert_eq!(self.expected.len(), self.actual.len(), "Expected {} nodes, but got {}. Actual nodes: {:?}", self.expected.len(), self.actual.len(), self.actual);

      for(index, (expected, actual)) in self.expected.iter().zip(
        self.actual.iter()
      ).enumerate() {
        assert_eq!(expected, actual, "Expected {:?} at index {}, but got {:?} instead.", expected, index, actual);
      }
    }
  }

  impl SyntaxTreeVisitor for SyntatTreeVerifier {
    fn visit_let_statement(&mut self, let_statement: &LetStatement) {
      self.actual.push(SyntaxTreeTestNode::LetStatement);
      self.visit_expression(&let_statement.initialiser);
    }

    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) {
      self.actual.push(SyntaxTreeTestNode::Variable(
        variable_expression.identifier().to_string()
      ));
    }

    fn visit_number(&mut self, number: &NumberExpression) {
      self.actual.push(SyntaxTreeTestNode::Number(number.number));
    }

    fn visit_error(&mut self, span: &TextSpan) {
      // Do Nothing 
      todo!()
    }

    fn visit_parenthesised_expression(&mut self, parenthesised_expression: &ParenthesisExpression) {
      self.actual.push(SyntaxTreeTestNode::Parenthesised);
      self.visit_expression(&parenthesised_expression.expression);
    }

    fn visit_binary_expression(&mut self, binary_expression: &BinaryExpression) {
      self.actual.push(SyntaxTreeTestNode::Binary);
      self.visit_expression(&binary_expression.left);
      self.visit_expression(&binary_expression.right);
    }  

    fn visit_unary_expression(&mut self, unary_expression: &UnaryExpression) {
      self.actual.push(SyntaxTreeTestNode::Unary);
      self.visit_expression(&unary_expression.operand);
    }
  }

  fn assert_tree(input: &str, expected: Vec<SyntaxTreeTestNode>) {
    let verifier = SyntatTreeVerifier::new(input, expected);
    verifier.verify();
  }

  #[test]
  pub fn should_parse_basic_binary_expression() {
    let input = "let a = 1 + 2";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2)
    ];
    assert_tree(input, expected);
  }

  #[test]
  pub fn should_parse_basic_parenthesised_expression() {
    let input = "let a = (1 + 2) * 3";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Parenthesised,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2),
      SyntaxTreeTestNode::Number(3)
    ];

    assert_tree(input, expected);
  }

  #[test]
  pub fn should_parse_basic_binary_expression_with_variable() {
    let input = "let a = (1 + 2) + b";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Parenthesised,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2),
      SyntaxTreeTestNode::Variable("b".to_string())
    ];

    assert_tree(input, expected);
  }

  #[test]
  pub fn should_parse_binary_exprssion_with_variable_and_number() {
    let input = "let a = (1 + 2) * b + 3";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Parenthesised,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2),
      SyntaxTreeTestNode::Variable("b".to_string()),      
      SyntaxTreeTestNode::Number(3)
    ];

    assert_tree(input, expected);
  }

  #[test]
  pub fn should_parse_bitwise_and() {
    let input = "let a = 1 & 2";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2)
    ];

    assert_tree(input, expected);
  }  

  #[test]
  pub fn should_parse_bitwise_or() {
    let input = "let a = 1 | 2";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2)
    ];

    assert_tree(input, expected);
  }

  #[test]
  pub fn should_parse_bitwise_xor() {
    let input = "let a = 1 ^ 2";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2)
    ];

    assert_tree(input, expected);
  }

  #[test]
  pub fn should_parse_negation() {
    let input = "let a = -1";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Unary,
      SyntaxTreeTestNode::Number(1)      
    ];

    assert_tree(input, expected);
  }

  #[test]
  pub fn should_parse_power() {
    let input = "let a = 1 ** 2";
    let expected = vec![
      SyntaxTreeTestNode::LetStatement,
      SyntaxTreeTestNode::Binary,
      SyntaxTreeTestNode::Number(1),
      SyntaxTreeTestNode::Number(2)      
    ];

    assert_tree(input, expected);
  }

}