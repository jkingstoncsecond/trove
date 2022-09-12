use core::panic;

use crate::lex::Token;
use crate::parse::Block;
use crate::parse::Identifier;
use crate::parse::ParsedAST;
use crate::parse::Program;
use crate::parse::Binary;

pub trait Generator {
    fn generate(&self);
}

pub struct CGenerator<'a> {
    pub ast: ParsedAST<'a>
}

impl Generator for CGenerator<'_> {
    fn generate(&self){
        println!("{:?}", self.ast);
        println!("===== generating =====");
        self.generate_ast(&self.ast);
        println!("\n======================");
    }
}

impl CGenerator<'_>{
    pub fn new(ast: ParsedAST) -> CGenerator {
        CGenerator {
           ast
        }
    }

    fn generate_ast<'a>(&self, ast: &ParsedAST){
         match ast {
             ParsedAST::PROGRAM(program) => {
                 self.generate_program(&program);
             },
             ParsedAST::BLOCK(block) => {
                 self.generate_block(&block);
             },
             ParsedAST::BINARY(binary) => {
                 self.generate_binary(&binary);
             },
             ParsedAST::IDENTIFIER(identifier) => {
                 self.generate_identifier(&identifier);
             },
             ParsedAST::NUMBER(number) => {
                self.generate_number(&number);
            },
             _ => panic!()
         }
    }

    fn generate_program<'a>(&self, program: &Program){
        for item in program.body.iter() {
            self.generate_ast(item);
        }
    }

    fn generate_block<'a>(&self, block: &Block){
        print!("{}", "{");
        for item in block.body.iter() {
            self.generate_ast(item);
        }
        print!("{}", "}");
    }

    fn generate_binary<'a>(&self, binary: &Binary){
        self.generate_ast(&binary.left);
        
        match binary.op {
            Token::PLUS => print!("+"),
            Token::MINUS => print!("-"),
            _ => panic!(),
        }
        
        self.generate_ast(&binary.right);

    }

    fn generate_identifier<'a>(&self, identifier: &Identifier){
        match identifier.token {
            Token::IDENTIFIER(value) => println!("{}", value),
            _ => panic!()
        }
    }

    fn generate_number<'a>(&self, number: &f32){
        print!("{}", number)
    }
}