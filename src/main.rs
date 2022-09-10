mod lex;
mod parse;

fn main() {

    let lexer = lex::Lexer{};
    let tokens = lexer.lex(&std::string::String::from("main pub fn"));

    let parser = parse::Parser{};
    let ast = parser.parse(tokens);

    println!("Hello, world!");
}
