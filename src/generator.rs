use core::panic;

use crate::lex::Token;
use crate::parse::Block;
use crate::parse::Call;
use crate::parse::Decl;
use crate::parse::Group;
use crate::parse::ParsedAST;
use crate::parse::Program;
use crate::parse::Binary;
use crate::typecheck::Mutability;
use crate::typecheck::Primative;
use crate::typecheck::Type;

pub trait Generator {
    fn generate(&self) -> std::string::String;
}

pub struct CGenerator<'a> {
    pub ast: &'a Box<ParsedAST<'a>>
}

impl Generator for CGenerator<'_> {
    fn generate(&self) -> std::string::String{
        println!("{:?}", self.ast);

        let mut code = "".to_string();

        self.emit(&mut code, "void test(const char* arg){printf(\"%s\\n\", arg);} void main(){".to_string());

        self.generate_ast(&mut code, &self.ast);

        self.emit(&mut code, ";}".to_string());

        println!("{}", code);

        code
    }
}

impl CGenerator<'_> {
    pub fn new<'a>(ast: &'a Box<ParsedAST>) -> CGenerator<'a> {
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
             ParsedAST::STMT(stmt) => {
                self.generate_ast(code, stmt);
                self.emit(code, ";".to_string());
             },
             ParsedAST::BLOCK(block) => {
                 self.generate_block(code, &block);
             },
             ParsedAST::DECL(decl) => {
                 self.generate_decl(code, &decl);
             },
             ParsedAST::BINARY(binary) => {
                 self.generate_binary(code, &binary);
             },
             ParsedAST::IDENTIFIER(identifier) => {
                 self.generate_identifier(code, identifier);
             },
             ParsedAST::STRING(string) => {
                 self.generate_string(code, string);
             },
             ParsedAST::GROUP(group) => {
                self.generate_group(code, &group);
             },
             ParsedAST::CALL(call) => {
                self.generate_call(code, &call);
             },
             ParsedAST::NUMBER(number) => {
                self.generate_number(code, &number);
             },
             _ => panic!()
         }
    }

    fn generate_type(&self, code: &mut std::string::String, typ: &Type){
        match typ {
            Type{mutability: Mutability::MUTABLE, ..} => {},
            Type{mutability: Mutability::CONSTANT, ..} => self.emit(code, "const ".to_string()),
            _ => panic!()
        }
        match typ {
            Type{primative: Primative::U32, ..} => self.emit(code, "unsigned int".to_string()),
            Type{primative: Primative::I32, ..} => self.emit(code, "int".to_string()),
            Type{primative: Primative::BOOL, ..} => self.emit(code, "unsigned int".to_string()),
            Type{primative: Primative::STRING, ..} => self.emit(code, "char*".to_string()),
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

    fn generate_decl<'a>(&self, code: &mut std::string::String, decl: &Decl){
        self.generate_type(code, &decl.typ);
        self.emit(code, " ".to_string());
        match decl.identifier {
            Token::IDENTIFIER(value) => self.emit(code, value.to_string()),
            _ => panic!()
        }
        self.emit(code, "=".to_string());
        self.generate_ast(code, &decl.value);
        self.emit(code, ";".to_string());
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

    fn generate_identifier<'a>(&self, code: &mut std::string::String, identifier: &std::string::String){
        self.emit(code, identifier.to_string())
    }

    fn generate_string<'a>(&self, code: &mut std::string::String, string: &std::string::String){
        self.emit(code, "\"".to_string());
        self.emit(code, string.to_string());
        self.emit(code, "\"".to_string());
    }

    fn generate_number<'a>(&self, code: &mut std::string::String, number: &f32){
        self.emit(code, number.to_string())
    }

    fn generate_group<'a>(&self, code: &mut std::string::String, group: &Group){
        self.emit(code, "(".to_string());
        self.generate_ast(code, &group.expression);
        self.emit(code, ")".to_string());
    }

    fn generate_call<'a>(&self, code: &mut std::string::String, call: &Call){
        self.generate_ast(code, &call.callee);
        self.emit(code, "(".to_string());
        self.generate_ast(code, &call.args);
        self.emit(code, ")".to_string());
    }
}