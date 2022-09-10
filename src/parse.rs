use crate::lex::Token;

pub struct Parser<'a>{
    pub tokens: &'a Box<Vec<Token>>
}

#[derive(Debug)]
pub struct Program<'a>{
    body: Vec<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Identifier<'a> {
    token: &'a Token
}

#[derive(Debug)]
pub enum ParsedAST<'a> {
    PROGRAM(Program<'a>),
    IDENTIFIER(Identifier<'a>)


}

// todo should the ast be an enum?


impl Parser<'_> {

    pub fn new(tokens: &mut Box<Vec<Token>>) -> Parser {
        Parser { tokens: tokens}
    }

    pub fn parse(&mut self){
        println!("parse!");
        let res = self.parse_program();
        println!("parse result {:?}.", res);
    }

    fn parse_program(&mut self) -> ParsedAST{
        println!("parsing program!");

        let mut current: usize = 0;
        let mut body: Vec<ParsedAST> = vec!();
        
        while !self.end(&current){
            match self.peek(&current) {
                Token::IDENTIFIER(_) => {
                    body.push(self.decl(&mut current));
                },
                _ => {
                    panic!("unknown token!");
                }
            }
        }
        
        return ParsedAST::PROGRAM(Program{body: body});
    }

    fn peek(&self, current: &usize) -> &Token {
        match self.tokens.get(*current) {
            std::option::Option::Some(t) => return t,
            _ => panic!("umm")
        }
    }

    fn end(&self, current: &usize) -> bool {
        *current >= self.tokens.len()
    }

    fn consume(&self, current: &mut usize) -> &Token {
        match self.tokens.get(*current) {
            std::option::Option::Some(t) => {
                *current+=1;
                return t;
            },
            _ => panic!("umm")
        }
    }

    fn decl(&self, counter: &mut usize) -> ParsedAST {
        let next = self.consume(counter);
        match next {
            Token::IDENTIFIER(_) => {
                // todo this is bad :(, we want a reference to the token
                return ParsedAST::IDENTIFIER(Identifier{token: next});
            },
            _ => {
                panic!()
            }
        }
    }
}