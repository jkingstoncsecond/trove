mod lex;
mod parse;

fn main() {

    let mut lexer = lex::Lexer::new();
    lexer.lex(Box::new(std::string::String::from("+-*/ helo umm")));

    let mut _parser = parse::Parser::new(&mut lexer.tokens);
    _parser.parse();
}
