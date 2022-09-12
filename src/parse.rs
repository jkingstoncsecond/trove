use crate::lex::Token;

#[derive(Debug)]
pub struct Program<'a>{
    body: Vec<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Block<'a>{
    body: Vec<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Identifier<'a> {
    token: &'a Token
}

#[derive(Debug)]
pub enum ParsedAST<'a> {
    PROGRAM(Program<'a>),
    BLOCK(Block<'a>),
    IDENTIFIER(Identifier<'a>)

}

pub struct Parser<'a>{
    pub tokens: &'a Box<Vec<Token>>,
    pub ast: ParsedAST<'a>
}
// todo should the ast be an enum?


impl Parser<'_> {

    pub fn new(tokens: &mut Box<Vec<Token>>) -> Parser {
        Parser { tokens: tokens, ast: ParsedAST::PROGRAM(Program{ body: vec!() })}
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
            body.push(self.statement(&mut current));
        }
        
        return ParsedAST::PROGRAM(Program{body: body});
    }

    fn statement(&self, current: &mut usize) -> ParsedAST {
        println!("statement! {:?}", self.peek(&current));
        match self.peek(&current) {
            Token::IDENTIFIER(_) => {
                return self.decl(current);
            },
            Token::LCURLY => {
                return self.block(current);
            },
            _ => {
                panic!("unknown token!");
            }
        }
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

    fn expecting(&self, token: Token, current: &usize) -> bool {
        let next = self.peek(current);
        return token.eq(&next);
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
        println!("doing decl:)");
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

    fn block(&self, current: &mut usize) -> ParsedAST {
        self.consume(current);
        let mut body: Vec<ParsedAST> = vec!();
        while !self.end(current) && !self.expecting(Token::RCURLY, current) {
            body.push(self.statement(current));
        }
        self.consume(current);
        return ParsedAST::BLOCK(Block{body});
    }
}