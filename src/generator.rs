use core::panic;

use crate::lex::Token;
use crate::parse::Block;
use crate::parse::Identifier;
use crate::parse::ParsedAST;
use crate::parse::Program;
use crate::parse::Binary;

pub trait Generator {
    fn generate(&self) -> std::string::String;
}

pub struct CGenerator<'a> {
    pub ast: ParsedAST<'a>
}

impl Generator for CGenerator<'_> {
    fn generate(&self) -> std::string::String{
        println!("{:?}", self.ast);

        let mut code = "".to_string();

        println!("===== generating =====");

        self.emit(&mut code, "void main(){printf(\"%d\\n\", ".to_string());

        self.generate_ast(&mut code, &self.ast);

        self.emit(&mut code, ");}".to_string());

        println!("\n======================");

        println!("{}", code);

        code
    }
}

impl CGenerator<'_>{
    pub fn new(ast: ParsedAST) -> CGenerator {
        CGenerator {ast}
    }

    fn emit(&self, code: &mut std::string::String, new_code: std::string::String) {
        code.push_str(new_code.as_str())   
    }

    fn generate_ast<'a>(&self, code: &mut std::string::String, ast: &ParsedAST){
         match ast {
             ParsedAST::PROGRAM(program) => {
                 self.generate_program(code, &program);
             },
             ParsedAST::BLOCK(block) => {
                 self.generate_block(code, &block);
             },
             ParsedAST::BINARY(binary) => {
                 self.generate_binary(code, &binary);
             },
             ParsedAST::IDENTIFIER(identifier) => {
                 self.generate_identifier(code, &identifier);
             },
             ParsedAST::NUMBER(number) => {
                self.generate_number(code, &number);
            },
             _ => panic!()
         }
    }

    fn generate_program<'a>(&self, code: &mut std::string::String, program: &Program){
        for item in program.body.iter() {
            self.generate_ast(code, item);
        }
    }

    fn generate_block<'a>(&self, code: &mut std::string::String, block: &Block){
        self.emit(code, "{".to_string());
        for item in block.body.iter() {
            self.generate_ast(code, item);
        }
        self.emit(code,"}".to_string());
    }

    fn generate_binary<'a>(&self, code: &mut std::string::String, binary: &Binary){
        self.generate_ast(code, &binary.left);
        
        match binary.op {
            Token::PLUS => self.emit(code, "+".to_string()),
            Token::MINUS => self.emit(code, "-".to_string()),
            Token::STAR => self.emit(code, "*".to_string()),
            Token::DIV => self.emit(code, "/".to_string()),
            _ => panic!(),
        }
        
        self.generate_ast(code, &binary.right);

    }

    fn generate_identifier<'a>(&self, code: &mut std::string::String, identifier: &Identifier){
        match identifier.token {
            Token::IDENTIFIER(value) => println!("{}", value),
            _ => panic!()
        }
    }

    fn generate_number<'a>(&self, code: &mut std::string::String, number: &f32){
        self.emit(code, number.to_string())
    }
}