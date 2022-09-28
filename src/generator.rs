use core::panic;

use crate::lex::Token;
use crate::parse::Assign;
use crate::parse::Block;
use crate::parse::Call;
use crate::parse::Decl;
use crate::parse::Directive;
use crate::parse::Fn;
use crate::parse::Group;
use crate::parse::If;
use crate::parse::LeftUnary;
use crate::parse::LhsAccess;
use crate::parse::Number;
use crate::parse::ParsedAST;
use crate::parse::Program;
use crate::parse::Binary;
use crate::parse::StructTypesList;
use crate::typecheck::Mutability;
use crate::typecheck::Primative;
use crate::typecheck::Type;

pub trait Generator {
    fn generate(&mut self) -> std::string::String;
}


#[derive(Debug)]
pub struct CodeBlock{
    pub statements: Vec<std::string::String>,
    pub index: usize
}

impl CodeBlock {

    pub fn new_stmt(&mut self){

        self.index+=1;
        self.statements.insert(self.index, "".to_string());
    }

    pub fn new_stmt_at(&mut self, offset: i32){
        self.index = (self.index as i32 + offset) as usize;
        self.statements.insert(self.index, "".to_string());
    }

    pub fn set_current(&mut self, statement: std::string::String){
        self.set_at(0, statement);
    }

    pub fn set_at_beginning(&mut self, statement: std::string::String){
        self.set_at(-(self.index as i32), statement)
    }

    pub fn set_at(&mut self, offset: i32, statement: std::string::String){
        self.statements.insert((self.index as i32 + offset) as usize, statement);
    }

    pub fn append_current(&mut self, code: std::string::String){
        let stmt = self.statements.get(self.index).unwrap();
        let mut new_string = stmt.to_string();
        new_string.push_str(code.as_str());
        self.statements[self.index] = new_string;
    }

    pub fn append_at_beginning(&mut self, code: std::string::String){
        let stmt = self.statements.get(0).unwrap();
        let mut new_string = stmt.to_string();
        new_string.push_str(code.as_str());
        self.statements[0] = new_string;
    }

    pub fn append_at(&mut self, offset: i32, code: std::string::String){
        let stmt = self.statements.get((self.index as i32 + offset) as usize).unwrap();
        let mut new_string = stmt.to_string();
        new_string.push_str(code.as_str());
        self.statements[(self.index as i32 + offset) as usize] = new_string;
    }
}

pub struct CGenerator<'a> {
    pub ast: &'a Box<ParsedAST<'a>>,
    pub blocks: Vec<CodeBlock>
}

impl Generator for CGenerator<'_> {
    fn generate(&mut self) -> std::string::String{

        let mut code = "".to_string();

        
        self.current_block().new_stmt();
        self.current_block().append_current("
        #include <cstdio>
        #include <cstdlib>
        
        void println(int* x){
            printf(\"ptr='%s'\\n\", (void*)x);
        } 

        void println(int x){
            printf(\"%d\\n\", x);
        } 
        
        void println(float x){
            printf(\"%f\\n\", x);
        } 

        void println(const char* arg){
            printf(\"%s\\n\", arg);
        }
        ".to_string());// int main(){".to_string());

        self.generate_ast(&mut code, &self.ast);

        for block in self.blocks.iter() {
            for stmt in block.statements.iter() {
                println!("---- {:?}", stmt);
                code.push_str(stmt);
            }
        }

        code
    }
}

impl CGenerator<'_> {
    pub fn new<'a>(ast: &'a Box<ParsedAST>) -> CGenerator<'a> {
        CGenerator {ast, blocks: vec![CodeBlock{statements: vec!["".to_string()], index: 0}]}
    }

    fn current_block(&mut self) -> &mut CodeBlock {
        &mut self.blocks[0]
    }

    fn emit_global(&mut self, code: &mut std::string::String, new_code: std::string::String) {
        
        // todo
        
        //code.push_str(new_code.as_str())   
    }

    // fn emit(&mut self, code: &mut std::string::String, new_code: std::string::String) {
    //     code.push_str(new_code.as_str())   
    // }

    fn generate_ast<'a>(&mut self, code: &mut std::string::String, ast: &ParsedAST){
         match ast {
            ParsedAST::DIRECTIVE(program) => {
                self.generate_directive(code, &program);
            },
             ParsedAST::PROGRAM(program) => {
                 self.generate_program(code, &program);
             },
             ParsedAST::FN(func) => {
                 self.generate_function(code, &func);
             },
             ParsedAST::STMT(stmt) => {
                self.current_block().new_stmt();
                self.generate_ast(code, stmt);
                // self.emit(code, ";\n".to_string());
                self.current_block().append_current(";\n".to_string());
             },
             ParsedAST::BLOCK(block) => {
                 self.generate_block(code, &block);
             },
             ParsedAST::IF(iff) => {
                 self.generate_if(code, &iff);
             },
             ParsedAST::RET(ret) => {
                 self.generate_ret(code, &ret);
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
             ParsedAST::FN(function) => {
                self.generate_function(code, &function);
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

    fn generate_type(&mut self, code: &mut std::string::String, typ: &Type){
        match typ {
            Type{primative: Primative::FN(func), ..} => {
                match typ.mutability {
                    Mutability::CONSTANT => {
                        // todo
                        self.current_block().append_current("void ".to_string());
                        self.current_block().append_current(func.anonymous_name.to_string());
                        self.current_block().append_current("() ".to_string());
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
                        self.current_block().append_current("struct ".to_string());
                        self.current_block().append_current(typeType.anonymous_name.to_string());
                    },
                    Mutability::MUTABLE => {
                        // todo
                    }
                }
            },
            _ => {
                match typ {
                    Type{mutability: Mutability::MUTABLE, ..} => {},
                    Type{mutability: Mutability::CONSTANT, ..} => self.current_block().append_current("const ".to_string()),
                    _ => panic!()
                }
                match typ {
                    Type{primative: Primative::U32, ..} => self.current_block().append_current("unsigned int".to_string()),
                    Type{primative: Primative::I32, ..} => self.current_block().append_current("int".to_string()),
                    Type{primative: Primative::F32, ..} => self.current_block().append_current("float".to_string()),
                    Type{primative: Primative::BOOL, ..} => self.current_block().append_current("unsigned int".to_string()),
                    Type{primative: Primative::STRING, ..} => self.current_block().append_current("char*".to_string()),
                    Type{primative: Primative::STRUCT(identifier), ..} => self.current_block().append_current(identifier.to_string()),
                    Type{primative: Primative::INCOMPLETE, ..} => self.current_block().append_current("void".to_string()),
                    // Type{primative: Primative::TYPE(typeType), ..} => {
                    //     self.emit(code, "struct ".to_string());
                    //     // todo get the struct anonymouse name
                    //     self.emit(code, typeType.anonymous_name.to_string());
                    // }, 
                    _ => panic!()
                }
                match typ.reference {
                    true => self.current_block().append_current("*".to_string()),
                    false => {}
                }
            }
        }
    }

    fn generate_directive<'a>(&mut self, code: &mut std::string::String, directive: &Directive){
        match &directive.value {
            Token::IDENTIFIER(identifier) => {
                match identifier.as_str() {
                    "inline" => {
                        self.current_block().append_current("inline ".to_string());
                        //self.generate_ast(code, &directive.body.unwrap());
                    },
                    "asm" => {
                        self.current_block().append_current("asm(\"".to_string());

                        match directive.args.get(0) {
                            Some(ParsedAST::STRING(s)) => {
                                self.current_block().append_current(s.to_string());
                            },
                            _ => panic!()
                        }

                        self.current_block().append_current("\")".to_string());
                    },
                    _ => {}
                }
            },
            _ => panic!()
        }
    }

    fn generate_program<'a>(&mut self, code: &mut std::string::String, program: &Program){
        for item in program.body.iter() {
            self.generate_ast(code, item);
        }
    }

    fn generate_block<'a>(&mut self, code: &mut std::string::String, block: &Block){
        if block.new_scope {
            self.current_block().new_stmt();
            self.current_block().append_current("{".to_string());
        }
        for item in block.body.iter() {
            self.generate_ast(code, item);
        }
        if block.new_scope {
            self.current_block().new_stmt();
            self.current_block().append_current("}".to_string());
        }
    }

    fn generate_if<'a>(&mut self, code: &mut std::string::String, iff: &If){
        self.current_block().append_current("if(".to_string());
        self.generate_ast(code, &iff.condition);
        self.current_block().append_current(")".to_string());
        self.generate_ast(code, &iff.body);
        match &iff.else_body {
            Some(else_body) => {
                self.current_block().append_current(" else ".to_string());
                self.generate_ast(code, &else_body);
            }
            None => {}
        }
    }

    fn generate_ret<'a>(&mut self, code: &mut std::string::String, ret: &Option<Box<ParsedAST<'a>>>){
        self.current_block().new_stmt();
        self.current_block().append_current("return ".to_string());
        match ret {
            Some(ast) => self.generate_ast(code, ast),
            None => {}
        }
        self.current_block().append_current(";\n".to_string());
    }

    fn generate_assign<'a>(&mut self, code: &mut std::string::String, assign: &Assign){
        self.generate_ast(code, &assign.lhs);
        self.current_block().append_current("=".to_string());
        self.generate_ast(code, &assign.rhs);
    }

    fn generate_decl<'a>(&mut self, code: &mut std::string::String, decl: &Decl){

        match &decl.typ.primative {
            Primative::FN(func) => {
                // todo
                // todo emit at global scope
                //self.generate_type(code, &decl.typ);
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
                self.current_block().append_current(" ".to_string());
                self.current_block().append_current(decl.identifier.to_string());
                match &decl.value {
                    Some(value) => {
                        self.current_block().append_current("=".to_string());
                        self.generate_ast(code, &value);
                    },
                    None => {}
                }
                // self.emit(code, ";".to_string());
            }
        }
    }

    fn generate_left_unary<'a>(&mut self, code: &mut std::string::String, left_unary: &LeftUnary){
        // todo
        match left_unary{
            LeftUnary::TAKE_REFERENCE(take_reference) => {

                if take_reference.rhs_type.reference {
                    self.current_block().append_current("*".to_string());
                    self.generate_ast(code, &take_reference.rhs);
                }else{
                    if(take_reference.is_heap_alloc){
                        self.current_block().new_stmt_at(0);
                        let mut lhs_type = take_reference.rhs_type.to_owned();
                        lhs_type.mutability = Mutability::MUTABLE;
                        self.generate_type(code, &lhs_type);
                        self.current_block().append_current("* tmp = (".to_string());
                        self.generate_type(code, &lhs_type);
                        self.current_block().append_current("*) malloc(sizeof(".to_string());
                        self.current_block().append_current(take_reference.rhs_type.size_in_bytes().to_string());
                        self.current_block().append_current("));\n".to_string());


                        // self.current_block().new_stmt_at(-1);
                        self.current_block().new_stmt();
                        self.current_block().append_current("*tmp = ".to_string());
                        self.generate_ast(code, &take_reference.rhs);
                        self.current_block().append_current(";".to_string());

                        // :(
                        self.current_block().index = self.current_block().index + 1;

                        self.current_block().append_current("tmp".to_string());
                    }else{
                        self.current_block().append_current("&".to_string());
                        self.generate_ast(code, &take_reference.rhs);
                    }
                }
            
                // todo check the rhs type, because we may wan't to dereference
            }
        }
    }

    fn generate_binary<'a>(&mut self, code: &mut std::string::String, binary: &Binary){
        self.generate_ast(code, &binary.left);
        
        match binary.op {
            Token::PLUS => self.current_block().append_current("+".to_string()),
            Token::MINUS => self.current_block().append_current("-".to_string()),
            Token::STAR => self.current_block().append_current("*".to_string()),
            Token::DIV => self.current_block().append_current("/".to_string()),
            _ => panic!(),
        }
        
        self.generate_ast(code, &binary.right);

    }

    fn generate_identifier<'a>(&mut self, code: &mut std::string::String, identifier: &std::string::String){
        self.current_block().append_current(identifier.to_string())
    }

    fn generate_string<'a>(&mut self, code: &mut std::string::String, string: &std::string::String){
        self.current_block().append_current("\"".to_string());
        self.current_block().append_current(string.to_string());
        self.current_block().append_current("\"".to_string());
    }

    fn generate_function<'a>(&mut self, code: &mut std::string::String, function: &Fn){
        //self.emit(code, number.to_string())
        self.current_block().new_stmt();
        match &function.typ.primative {
            Primative::FN(fun) => {
                match &fun.return_type {
                    Some(return_type) => self.generate_type(code, &return_type),
                    None => self.current_block().append_current("void".to_string())
                }
            },
            _ => panic!()
        }
        self.current_block().append_current(" ".to_string());
        match &function.typ.primative {
            Primative::FN(func) => {
                self.current_block().append_current(func.anonymous_name.to_string());
            },
            _ => panic!()
        }
        self.current_block().append_current("(".to_string());
        for (i, param) in function.params.iter().enumerate() {
            self.generate_decl(code, &param);
            if i < function.params.len() -1 {
                self.current_block().append_current(", ".to_string());
            }
        }
        self.current_block().append_current(")".to_string());
        self.current_block().append_current("{".to_string());

        self.current_block().new_stmt();
        self.generate_ast(code, &function.body);

        self.current_block().new_stmt();
        self.current_block().append_current("}".to_string());

    }

    fn generate_number<'a>(&mut self, code: &mut std::string::String, number: &Number){
        match number {
            Number::INTEGER(num) => self.current_block().append_current(num.to_string()),
            Number::FLOAT(num) => self.current_block().append_current(num.to_string()),
        }
    }

    fn generate_group<'a>(&mut self, code: &mut std::string::String, group: &Group){
        self.current_block().append_current("(".to_string());
        self.generate_ast(code, &group.expression);
        self.current_block().append_current(")".to_string());
    }

    fn generate_call<'a>(&mut self, code: &mut std::string::String, call: &Call){
        self.generate_ast(code, &call.callee);
        self.current_block().append_current("(".to_string());
        for (i, arg) in call.args.iter().enumerate() {
            self.generate_ast(code, arg);
            if i < call.args.len() - 1 {
                self.current_block().append_current(", ".to_string());
            }
        }
        self.current_block().append_current(")".to_string());
    }

    fn generate_struct_types_list<'a>(&mut self, code: &mut std::string::String, struct_types_list: &StructTypesList){
        self.current_block().append_current("{".to_string());
        for decl in struct_types_list.types.iter() {
            self.generate_decl(code, decl);
            self.current_block().append_current(";\n".to_string());
        }
        self.current_block().append_current("}".to_string());
    }

    fn generate_lhs_access<'a>(&mut self, code: &mut std::string::String, lhs_access: &LhsAccess){
        self.generate_ast(code, &lhs_access.left);
        self.current_block().append_current(".".to_string());
        self.generate_ast(code, &lhs_access.right);
    }
}