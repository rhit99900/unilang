use crate::syntax::SyntaxTree;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::syntax::evaluator::Evaluator;

mod syntax;
mod diagnostics;

fn main() {
    let input = "(1+1) * 7 + (8 * 9)";
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

    // Parsing Tokens
    let mut syntax_tree: SyntaxTree = SyntaxTree::new();
    let mut parser = Parser::new(tokens);

    // Parse Statements 
    while let Some(stmt) = parser.next_statement() {
        syntax_tree.add_statement(stmt);
    }
    
    syntax_tree.visualise();
    let mut evaluate = Evaluator::new();
    syntax_tree.visit(&mut evaluate);
    println!("Result: {:?}", evaluate.last_value);
}
