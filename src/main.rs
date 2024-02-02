
mod syntax;

fn main() {
    let input = "7";
    let mut lexer = syntax::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }

    println!("{:?}", tokens);
}
