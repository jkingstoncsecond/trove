use crate::parse::{ParsedAST, Program, Decl};


#[derive(Debug)]
pub struct Fn{
    
}

#[derive(Debug)]
pub enum Primative{
    NONE,
    U32,
    I32,
    BOOL,
    STRING,
    FN(Fn),
    BLOCK
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
            ParsedAST::DECL(decl) => self.type_check_decl(decl),
            ParsedAST::NUMBER(num) => self.type_check_num(num),
            ParsedAST::STRING(s) => self.type_check_string(s),
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
}