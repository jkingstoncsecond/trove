use crate::{parse::{ParsedAST, Program, Decl, Binary, Block}, lex::Token};

#[derive(Debug)]
pub struct SymTable{
    
}

#[derive(Debug)]
pub struct Fn{
    pub anonymous_name: std::string::String,
    pub args: Vec<Type>,
    // pub return_type: Type
}

#[derive(Debug)]
pub enum Primative{
    NONE,
    U32,
    I32,
    BOOL,
    STRING,
    FN(Fn),
    BLOCK,
    TYPE
}

#[derive(Debug)]
pub enum Mutability {
    MUTABLE,
    CONSTANT
}

#[derive(Debug)]
pub struct Type {
    pub mutability: Mutability,
    pub primative: Primative
}

pub struct TypeChecker {
    // pub ast: Box<ParsedAST<'a>>
}

impl TypeChecker {
    pub fn new () -> TypeChecker {//(ast: Box<ParsedAST>) -> TypeChecker {
        // TypeChecker { ast }
        TypeChecker { }
    }

    pub fn type_check<'a>(&mut self, mut tmp: Box<ParsedAST<'a>>) -> Box<ParsedAST<'a>> {
        //println!("typechecking...");
        self.type_check_ast(tmp.as_mut()); // hmm can we do this without it being mutable? NO!
        //let x: ParsedAST<'a> = *tmp;
        //Box::new(x)
        tmp
    }

    fn type_check_ast(&mut self, ast: &mut ParsedAST) -> Option<Type> {
        match ast {
            ParsedAST::STMT(stmt) => self.type_check_ast(stmt),
            ParsedAST::PROGRAM(program) => self.type_check_program(program),
            ParsedAST::BLOCK(block) => self.type_check_block(block),
            ParsedAST::DECL(decl) => self.type_check_decl(decl),
            ParsedAST::NUMBER(num) => self.type_check_num(num),
            ParsedAST::STRING(s) => self.type_check_string(s),
            ParsedAST::BINARY(binary) => self.type_check_binary(binary),
            ParsedAST::CALL(s) => None, // todo
            _ => panic!()
        }
    }
    
    // todo should type be optional
    fn type_check_program(&mut self, program: &mut Program) -> Option<Type> {
        for item in program.body.iter_mut() {
            self.type_check_ast(item);
        }
        // todo 
        None
    }

    fn type_check_block(&mut self, block: &mut Block) -> Option<Type> {
        for item in block.body.iter_mut() {
            self.type_check_ast(item);
        }
        // todo 
        None
    }

    fn type_check_decl(&mut self, decl: &mut Decl) -> Option<Type> {

        // todo
        if decl.requires_infering {
            let value_type = self.type_check_ast(&mut decl.value);
            println!("got value type as {:?}.", value_type);
            match value_type {
                Some(t) => decl.typ = t,
                None => panic!()
            }
        }

        match &mut decl.typ {
            Type{primative: Primative::FN(func), ..} => {
                match decl.identifier {
                    Token::IDENTIFIER(identifier) => {
                        func.anonymous_name = identifier.to_string()
                    },
                    _ => panic!()
                }
            }
            _ => {}
        }

        None
    }

    fn type_check_num(&self, num: &f32) -> Option<Type> {
        // todo
        Some(Type { mutability: Mutability::CONSTANT, primative: Primative::I32 })
    }

    fn type_check_string(&self, s: &std::string::String) -> Option<Type> {
        // todo
        // todo primitive strings!
        Some(Type { mutability: Mutability::CONSTANT, primative: Primative::STRING })
    }

    fn type_check_binary(&mut self, binary: &mut Binary) -> Option<Type> {
        // todo for now we just return the lhs
        return match self.type_check_ast(&mut binary.left) {
            Some(typ) => Some(typ),
            None => panic!()
        }
        
        // todo
        // todo primitive strings!
        //
        //Some(Type { mutability: Mutability::CONSTANT, primative: Primative::STRING })
    }
}