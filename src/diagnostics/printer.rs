use std::cmp;
use std::fmt::format;
use termion::color::{Fg, Red, Reset};
use crate::diagnostics::Diagnostic;
use crate::text::SourceText;


pub struct DiagnosticPrinter<'a> {
  text: &'a SourceText,
  diagnostics: &'a [Diagnostic]
}

const PREFIX_LENGTH: usize = 8;

impl <'a> DiagnosticPrinter<'a> {
  pub fn new(text: &'a SourceText, diagnostics: &'a [Diagnostic]) -> Self {
    return Self { text, diagnostics }
  }

  pub fn stringify_diagnostics(&self, diagnostic: &Diagnostic) -> String {
    let line_index = self.text.line_index(diagnostic.span.start);
    let line = self.text.get_line(line_index);

    let line_start = self.text.line_start(line_index);
    let column = diagnostic.span.start - line_start;
    let (prefix, span, suffix) = self.text_spans(diagnostic, &line, column);

    let indent = cmp::min(PREFIX_LENGTH, column);
    let (pointers, pointer_line) = Self::format_pointer(diagnostic, indent);
    let error_message = Self::format_error_message(diagnostic, indent);
    return format!("{}{}{}{}{}\n{}\n{}\n{}", prefix, Fg(Red), span, Fg(Reset), suffix, pointers, pointer_line, error_message);
  }

  fn format_error_message(diagnostic: &Diagnostic, indent: usize) -> String {
    return format!("{:indent$}+-- {}", "", diagnostic.message, indent = indent);
  }

  fn format_pointer(diagnostic: &Diagnostic, indent: usize) -> (String, String) {
    let pointers = format!("{:indent$}{}", "", std::iter::repeat('^')
      .take(
        diagnostic.span.length()
      )
      .collect::<String>(), indent = indent);
    let pointer_line = format!("{:indent$}|", "", indent = indent);
    return (pointers, pointer_line);
  }

  fn text_spans(&'a self, diagnostic: &Diagnostic, line: &'a str, column: usize) -> (&'a str, &'a str, &'a str) {
    let prefix_start = cmp::max(0, column as isize - PREFIX_LENGTH as isize) as usize;
    let prefix_end = column;
    let suffix_start = cmp::min(column + diagnostic.span.length(), line.len());
    let suffix_end = cmp::min(suffix_start + PREFIX_LENGTH, line.len());

    let prefix = &line[prefix_start..prefix_end];
    let span = &line[prefix_end..suffix_start];
    let suffix = &line[suffix_start..suffix_end];
    
    return(prefix, span, suffix);
  }

  pub fn print(&self) {
    for diagnostic in self.diagnostics {
      println!("{}", self.stringify_diagnostics(diagnostic));
    }
  }  
}