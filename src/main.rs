mod lexer;
mod parser;

fn main() {
    let mut tokens = lexer::tokenize("print(\"Hello, World!\" + (2 * 7));".to_string());
    lexer::print_tokens(&mut tokens);
    parser::parse(&mut tokens);
}
