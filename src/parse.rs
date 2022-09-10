use crate::lex::Token;

pub struct Parser{

}

impl Parser {
    pub fn parse(&self, tokens: Box<Vec<Token>>){
        for item in tokens.iter(){
            println!("token {:?}.", item);
        }
    }
}