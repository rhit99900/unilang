use crate::syntax::SyntaxTree;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;

mod syntax;

fn main() {
    let input = "(7 + 8) * 9";
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }

    // Printing Tokens tokenised by Lexer;
    println!("{:?}", tokens);

    // Parsing Tokens
    let mut syntax_tree: SyntaxTree = SyntaxTree::new();
    let mut parser = Parser::new(tokens);

    // Parse Statements 
    while let Some(stmt) = parser.next_statement() {
        syntax_tree.add_statement(stmt);
    }
    
    syntax_tree.visualise();
    
}
