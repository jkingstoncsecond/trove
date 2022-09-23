use core::panic;

use crate::lex::Token;
use crate::parse::Assign;
use crate::parse::Block;
use crate::parse::Call;
use crate::parse::Decl;
use crate::parse::Group;
use crate::parse::If;
use crate::parse::LeftUnary;
use crate::parse::LhsAccess;
use crate::parse::ParsedAST;
use crate::parse::Program;
use crate::parse::Binary;
use crate::parse::StructTypesList;
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

        let mut code = "".to_string();

        self.emit(&mut code, "
        #include <cstdio>
        
        void println(int* x){
            printf(\"ptr='%s'\\n\", (void*)x);
        } 
        
        void println(int x){
            printf(\"%d\\n\", x);
        } 
        
        void println(const char* arg){
            printf(\"%s\\n\", arg);
        }
        ".to_string());// int main(){".to_string());

        self.generate_ast(&mut code, &self.ast);

        println!("{}", code);

        code
    }
}

impl CGenerator<'_> {
    pub fn new<'a>(ast: &'a Box<ParsedAST>) -> CGenerator<'a> {
        CGenerator {ast}
    }

    fn emit_global(&self, code: &mut std::string::String, new_code: std::string::String) {
        
        // todo
        
        //code.push_str(new_code.as_str())   
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
             ParsedAST::IF(iff) => {
                 self.generate_if(code, &iff);
             },
             ParsedAST::DECL(decl) => {
                 self.generate_decl(code, &decl);
             },
             ParsedAST::ASSIGN(assign) => {
                 self.generate_assign(code, &assign);
             },
             ParsedAST::LEFT_UNARY(left_unary) => {
                 self.generate_left_unary(code, &left_unary);
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
             ParsedAST::STRUCT_TYPES_LIST(struct_types_list) => {
                self.generate_struct_types_list(code, &struct_types_list);
             },
             ParsedAST::LHS_ACCESS(lhs_access) => {
                self.generate_lhs_access(code, &lhs_access);
             },
             _ => panic!()
         }
    }

    fn generate_type(&self, code: &mut std::string::String, typ: &Type){
        match typ {
            Type{primative: Primative::FN(func), ..} => {
                match typ.mutability {
                    Mutability::CONSTANT => {
                        // todo
                        self.emit(code, "int ".to_string());
                        self.emit(code, func.anonymous_name.to_string());
                        self.emit(code, "() ".to_string());
                    },
                    Mutability::MUTABLE => {
                        // todo
                    }
                }
            },
            Type{primative: Primative::TYPE(typeType), ..} => {
                match typ.mutability {
                    Mutability::CONSTANT => {
                        // todo
                        self.emit(code, "struct ".to_string());
                        self.emit(code, typeType.anonymous_name.to_string());
                    },
                    Mutability::MUTABLE => {
                        // todo
                    }
                }
            },
            _ => {
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
                    Type{primative: Primative::STRUCT(identifier), ..} => self.emit(code, identifier.to_string()),
                    // Type{primative: Primative::TYPE(typeType), ..} => {
                    //     self.emit(code, "struct ".to_string());
                    //     // todo get the struct anonymouse name
                    //     self.emit(code, typeType.anonymous_name.to_string());
                    // }, 
                    _ => panic!()
                }
                match typ.reference {
                    true => self.emit(code, "*".to_string()),
                    false => {}
                }
            }
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

    fn generate_if<'a>(&self, code: &mut std::string::String, iff: &If){
        self.emit(code, "if(".to_string());
        self.generate_ast(code, &iff.condition);
        self.emit(code, ")".to_string());
        self.generate_ast(code, &iff.body);
        match &iff.else_body {
            Some(else_body) => {
                self.emit(code, " else ".to_string());
                self.generate_ast(code, &else_body);
            }
            None => {}
        }
    }

    fn generate_assign<'a>(&self, code: &mut std::string::String, assign: &Assign){
        self.generate_ast(code, &assign.lhs);
        self.emit(code, "=".to_string());
        self.generate_ast(code, &assign.rhs);
    }

    fn generate_decl<'a>(&self, code: &mut std::string::String, decl: &Decl){
        match &decl.typ.primative {
            Primative::FN(func) => {
                // todo
                // todo emit at global scope
                self.generate_type(code, &decl.typ);
                match &decl.value {
                    Some(value) => self.generate_ast(code, &value),
                    None => {}
                }
            },
            Primative::TYPE(func) => {
                // todo
                // todo emit at global scope
                self.generate_type(code, &decl.typ);
                match &decl.value {
                    Some(value) => self.generate_ast(code, &value),
                    None => {}
                }
            },
            _ => {
                self.generate_type(code, &decl.typ);
                self.emit(code, " ".to_string());
                match decl.identifier {
                    Token::IDENTIFIER(value) => self.emit(code, value.to_string()),
                    _ => panic!()
                }
                match &decl.value {
                    Some(value) => {
                        self.emit(code, "=".to_string());
                        self.generate_ast(code, &value);
                    },
                    None => {}
                }
                self.emit(code, ";".to_string());
            }
        }
    }

    fn generate_left_unary<'a>(&self, code: &mut std::string::String, left_unary: &LeftUnary){
        // todo
        match left_unary{
            LeftUnary::TAKE_REFERENCE(value) => {
                self.emit(code, "&".to_string());
                self.generate_ast(code, &value);
            }
        }
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
        match &call.args {
            Some(args) => self.generate_ast(code, &args),
            None => {}
        }
        self.emit(code, ")".to_string());
    }

    fn generate_struct_types_list<'a>(&self, code: &mut std::string::String, struct_types_list: &StructTypesList){
        self.emit(code, "{".to_string());
        for decl in struct_types_list.types.iter() {
            self.generate_decl(code, decl);
        }
        self.emit(code, "}".to_string());
    }

    fn generate_lhs_access<'a>(&self, code: &mut std::string::String, lhs_access: &LhsAccess){
        self.generate_ast(code, &lhs_access.left);
        self.emit(code, ".".to_string());
        self.generate_ast(code, &lhs_access.right);
    }
}