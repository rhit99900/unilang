use std::cell::RefCell;
use std::rc::Rc;

use diagnostics::printer::DiagnosticPrinter;
use diagnostics::DiagnosticGlossaryCell;
use text::SourceText;

use crate::syntax::SyntaxTree;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::syntax::evaluator::Evaluator;

mod syntax;
mod diagnostics;
mod text;
mod symbols;

fn main() -> Result<(), ()> {
	let input = "\
		let a = 10+30
		let b = 20		
		let d = 10 + e
		let c = (a + b) * d
	";
	let text = text::SourceText::new(input.to_string());
	let mut lexer = Lexer::new(input); 
	let mut tokens = Vec::new();
	while let Some(token) = lexer.next_token(){
			tokens.push(token);
	}

	// Printing Tokens tokenised by Lexer;
	// Debug Logs
	// for token in &tokens {
	// 		println!("{:?}", token);
	// }

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

	diagnose(&text, &_diagnostic)?;
	let mut symbol_checker = symbols::SymbolChecker::new(Rc::clone(&_diagnostic));
	syntax_tree.visit(&mut symbol_checker);
	diagnose(&text, &_diagnostic)?;
	let mut evaluate = Evaluator::new();
	syntax_tree.visit(&mut evaluate);
	println!("Result: {:?}", evaluate.last_value);
	Ok(())
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