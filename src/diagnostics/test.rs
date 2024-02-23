#[cfg(test)]
mod test {
  use crate::{
    compilation_unit::CompilationUnit, 
    diagnostics::{
      Diagnostic,
      DiagnosticKind
    },
    syntax::lexer::TextSpan
  };

  struct DiagnosticVerifier {
    actual: Vec<Diagnostic>,
    expected: Vec<Diagnostic>
  }

  impl DiagnosticVerifier {
    pub fn new(input: &str, messages: Vec<&str>) -> Self {
      let expected = Self::parse_input(input, messages);
      let actual = Self::compile(input);
      Self { expected, actual }
    }

    fn compile(input: &str) -> Vec<Diagnostic> {
      let raw_text = Self::get_raw_text(input);
      let compilation_unit = CompilationUnit::compile(&raw_text);
      let diagnostics = compilation_unit.diagnostic_glossary.borrow();
      return diagnostics.diagnostics.clone();
    }

    fn get_raw_text(input: &str) -> String {
      return input.replace("<", "").replace(">", "");
    }

    fn parse_input(input: &str, messages: Vec<&str>) -> Vec<Diagnostic> {
      let raw_text = Self::get_raw_text(input);
      let mut start_index_stack = vec![];
      let mut current_position: usize = 0;
      let mut diagnostics = vec![];

      for c in input.chars() {
        match c {
          '<' => {
            start_index_stack.push(current_position);
          },
          '>' => {
            let start_index = start_index_stack.pop().unwrap();
            let end_index = current_position;
            let literal = &raw_text[start_index..end_index];
            let span = TextSpan::new(start_index, end_index, literal.to_string());
            let message = messages[diagnostics.len()].to_string();
            let diagnostic = Diagnostic::new(message, span, DiagnosticKind::Error);
            diagnostics.push(diagnostic);
          },
          _ => {
            current_position += 1;
          }
        }        
      }

      return diagnostics;
    }

    pub fn verify(&self) {
      assert_eq!(self.actual.len(), self.expected.len(), "Expected {} diagnostics, found {} instead", self.expected.len(), self.actual.len());
      for (actual, expected) in self.actual.iter().zip(self.expected.iter()) {
        assert_eq!(actual.message, expected.message, "Expected message '{}', found '{}' instead", expected.message, actual.message);
        assert_eq!(actual.span.start, expected.span.start, "Expected start index {}, found {} instead", actual.span.start, expected.span.start);
        assert_eq!(actual.span.end, expected.span.end, "Expected end index {}, found {} instead", actual.span.end, expected.span.end);
        assert_eq!(actual.span.literal, expected.span.literal, "Expected literal index {}, found {} instead", actual.span.literal, expected.span.literal);
      }
    }

  }

  #[test]
  fn should_report_undeclared_variable() {
    let input = "let a = <b>";
    let expected = vec![
      "Undeclared variable 'b'"
    ];

    let verifier = DiagnosticVerifier::new(input, expected);
    verifier.verify();
  }

  #[test]
  fn should_report_expected_expression() {
    let input = "let a = <+>";
    let expected = vec![
      "Expected expression, found <+>"
    ];
    let verifier = DiagnosticVerifier::new(input, expected);
    verifier.verify();
  }

  #[test]
  fn should_report_bad_token() {
    let input = "let a = 8 <@> 2";
    let expected = vec![
      "Expected expression, found <Bad>"
    ];

    let verifier = DiagnosticVerifier::new(input, expected);
    verifier.verify();
  }
} 