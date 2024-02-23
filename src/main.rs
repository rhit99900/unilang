use compilation_unit::CompilationUnit;

// use termion::{raw::IntoRawMode, input::TermRead, event::Key};
// use std::{fmt::write, io::{self, Read, Write}};

mod syntax;
mod diagnostics;
mod text;
mod symbols;
mod compilation_unit;

fn main() {
	let input = "\
		let a = 10+30
		let b = 20		
		let d = 10 + e
		let c = (a + b) * d
	";	
	
	let compilation_unit = CompilationUnit::compile(input);
	compilation_unit.run();

	// let mut stdout = io::stdout().into_raw_mode().unwrap();
	// let stdin = io::stdin();

	// writeln!(stdout, "Unilang IDE Version 0.0.1\r").unwrap();
	// stdout.flush().unwrap();

	// for c in stdin.keys() {
	// 	match c.unwrap() {
	// 		Key::Esc => break,
	// 		Key::Backspace => {
	// 			println!("Exiting!\r");
	// 		},
	// 		Key::Char(c) => {
	// 			write!(stdout, "{}", c).unwrap()
	// 		},
	// 		_ => {}
	// 	}
	// 	stdout.flush().unwrap();
	// }

}