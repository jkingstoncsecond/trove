use crate::parse::ParsedAST;

pub struct Analyser {
    
}

impl Analyser{
    pub fn analyse<'a>(&mut self, mut tmp: Box<ParsedAST<'a>>) -> Box<ParsedAST<'a>> {
        println!("-------------- analysing!");
        tmp
    }
}