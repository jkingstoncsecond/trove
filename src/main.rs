mod lex;
mod parse;

fn main() {

    let mut lexer = lex::Lexer{current: 0};
    let tokens = lexer.lex(&std::string::String::from("+-*/ helo umm"));

    let mut parser = parse::Parser{};
    let _ast = parser.parse(tokens);

    // todo generate


    println!("Hello, world!");
}
