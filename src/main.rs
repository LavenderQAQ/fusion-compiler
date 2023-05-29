use crate::ast::lexer::Lexer;
use crate::ast::parser::Parser;
use crate::ast::Ast;

mod ast;

fn main() {
    let input = "6 + (7 * 8)";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);

    let mut ast = Ast::new();
    let mut parser = Parser::new(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement)
    }

    ast.visualize();
}
