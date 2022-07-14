mod emit;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let mut tokens = lexer::tokenize("if (2 > 3) { print(\"something\") }".to_string());
    lexer::print_tokens(&mut tokens);
    let module = emit::build_module(parser::parse(&mut tokens));
    let mut vm = runtime::vm::VMContext::new();
    vm.run(&module);
}
