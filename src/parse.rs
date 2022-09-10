use crate::lex::Token;

pub struct Parser<'a>{
    pub current: usize,
    pub tokens: &'a Box<Vec<Token>>
}

pub struct Program{
}

// todo should the ast be an enum?


impl Parser<'_> {

    pub fn new(tokens: &mut Box<Vec<Token>>) -> Parser {
        Parser { current: 0, tokens: tokens}
    }

    pub fn parse(&mut self){

        println!("parse!");
        for item in self.tokens.iter(){
            println!("token {:?}.", item);

            //self.parse_program();

        }
    }

    fn parse_program(&self){
        println!("parsing program! {}.", self.peek())
    }

    fn peek(&self) -> &Token {
        match self.tokens.get(self.current) {
            std::option::Option::Some(t) => return t,
            _ => panic!("umm")
        }
    }
}