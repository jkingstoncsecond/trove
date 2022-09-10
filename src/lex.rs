
#[derive(Debug)]
pub enum Token {
    END,
    NUMBER,
    STRING,
    IDENTIFIER
}

pub struct Lexer {

}

impl Lexer {
    pub fn lex(&self, program: &std::string::String) -> Box<Vec<Token>>{
        println!("lexing {}.", &program);
        return Box::new(vec!(Token::IDENTIFIER, Token::END));
    }
}