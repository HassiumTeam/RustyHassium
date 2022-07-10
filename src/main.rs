mod lexer;
mod parser;

fn main() {
    let mut tokens = lexer::tokenize("if (2 > 3) { print(\"something\") }".to_string());
    lexer::print_tokens(&mut tokens);
    parser::parse(&mut tokens);
}
