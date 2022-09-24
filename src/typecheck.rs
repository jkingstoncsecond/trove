use std::hash::Hash;

use crate::{parse::{ParsedAST, Program, Decl, Binary, Block, Fn as ParsedFn, If, Assign, LeftUnary, Call, Directive}, lex::Token};

#[derive(Debug)]
pub struct SymTable<K,T>{
    scope: usize,
    symbols: Vec<std::collections::HashMap<K, T>>
}

impl<K,T> SymTable<K,T> where K: Eq + Hash {

    pub fn add(&mut self, key: K, item: T){
        match self.symbols.get_mut(self.scope) {
            Some(map) => map.insert(key, item),
            None => panic!()
        };
    }

    pub fn get(&self, key: K) -> Option<&T> {
        return match self.symbols.get(self.scope) {
            Some(map) => map.get(&key),
            None => panic!()
        }
    }

    pub fn new_scope(&mut self){
        self.scope+=1;
        self.symbols.push(std::collections::HashMap::new());
    }

    pub fn leave_scope(&mut self){
        self.scope-=1;
        self.symbols.pop();
    }

}

#[derive(Debug, Clone)]
pub struct Fn{
    pub anonymous_name: std::string::String,
    pub args: Vec<Type>,
    pub return_type: Option<Box<Type>>
}

#[derive(Debug, Clone)]
pub struct TypeType{
    pub anonymous_name: std::string::String
}

#[derive(Debug, Clone)]
pub enum Primative{
    INCOMPLETE,
    U32,
    I32,
    BOOL,
    STRING,
    FN(Fn),
    BLOCK,
    TYPE(TypeType),
    STRUCT(std::string::String)
}

#[derive(Debug, Clone, Copy)]
pub enum Mutability {
    MUTABLE,
    CONSTANT
}

#[derive(Debug, Clone)]
pub struct Type {
    pub mutability: Mutability,
    pub primative: Primative,
    pub reference: bool
}

pub struct TypeChecker {
    // pub ast: Box<ParsedAST<'a>>
    pub sym_table: SymTable<std::string::String, Type>
}

impl TypeChecker {
    pub fn new () -> TypeChecker {//(ast: Box<ParsedAST>) -> TypeChecker {
        // TypeChecker { ast }
        TypeChecker { sym_table: SymTable { scope: 0, symbols: vec!(std::collections::HashMap::new()) }}
    }

    pub fn type_check<'a>(&mut self, mut tmp: Box<ParsedAST<'a>>) -> Box<ParsedAST<'a>> {
        println!("typechecking...");

        self.type_check_ast(tmp.as_mut()); // hmm can we do this without it being mutable? NO!
        //let x: ParsedAST<'a> = *tmp;
        //Box::new(x)
        tmp
    }

    fn type_check_ast(&mut self, ast: &mut ParsedAST) -> Option<Type> {
        //println!("... ast {:?}.", ast);
        match ast {
            ParsedAST::DIRECTIVE(directive) => self.type_check_directive(directive),
            ParsedAST::STMT(stmt) => self.type_check_ast(stmt),
            ParsedAST::PROGRAM(program) => self.type_check_program(program),
            ParsedAST::BLOCK(block) => self.type_check_block(block),
            ParsedAST::IF(iff) => self.type_check_if(iff),
            ParsedAST::DECL(decl) => self.type_check_decl(decl),
            ParsedAST::ASSIGN(assign) => self.type_check_assign(assign),
            ParsedAST::FN(func) => self.type_check_func(func),
            ParsedAST::NUMBER(num) => self.type_check_num(num),
            ParsedAST::IDENTIFIER(identifier) => self.type_check_identifier(identifier),
            ParsedAST::STRING(s) => self.type_check_string(s),
            ParsedAST::LEFT_UNARY(left_unary) => self.type_check_left_unary(left_unary),//self.type_check_binary(binary),
            ParsedAST::BINARY(binary) => None,//self.type_check_binary(binary),
            ParsedAST::CALL(call) => self.type_check_call(call), // todo
            ParsedAST::STRUCT_TYPES_LIST(s) => None, // todo
            ParsedAST::LHS_ACCESS(lhs_access) => None, // todo
            ParsedAST::GROUP(_) => None, // todo
            _ => panic!()
        }
    }
    
    fn type_check_directive(&mut self, directive: &mut Directive) -> Option<Type> {
        self.type_check_ast(&mut directive.body)
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
        // println!("... block {:?}.", block);
        for item in block.body.iter_mut() {
            self.type_check_ast(item);
        }
        // todo 
        None
    }

    fn type_check_if(&mut self, iff: &mut If) -> Option<Type> {
        // println!("... if {:?}.", iff);
        self.type_check_ast(&mut iff.condition);
        self.type_check_ast(&mut iff.body);
        match &mut iff.else_body {
            Some(body) => {self.type_check_ast(body);},
            None => {}
        };
        // todo 
        None
    }

    fn type_check_assign(&mut self, assign: &mut Assign) -> Option<Type> {
        self.type_check_ast(&mut assign.lhs);
        self.type_check_ast(&mut assign.rhs);
        None
    }

    fn type_check_decl(&mut self, decl: &mut Decl) -> Option<Type> {

        let mut decl_identifier: std::string::String;
        match decl.identifier {
            Token::IDENTIFIER(identifier) => decl_identifier = identifier.to_owned(),
            _ => panic!()
        }

        //println!("typecheck decl! {:?}", decl);
        // todo check if symtable contains key
        // match decl.identifi.er {
        //     Token::IDENTIFIER(identifier) => {
        //         match self.sym_table.get(identifier.to_string()) {
        //             Some(_) => panic!("symbol already declared!"),
        //             None => self.sym_table.add(identifier.to_string(), decl.typ)// todo
        //         }
        //     },
        //     _ => panic!()
        // }

        match &mut decl.value {
            Some(val) => {
                match val.as_mut() {
                    ParsedAST::FN(func) => {
                        let mut func_prim = &mut func.typ.primative;
                        match func_prim {
                            Primative::FN(prim_fn) => {
                                prim_fn.anonymous_name = decl_identifier.to_string();
                            },
                            _ => panic!()
                        }
                        decl.typ = func.typ.to_owned();
                    },
                    _ => {
                        let value_type = self.type_check_ast(val);
                        if decl.requires_infering {
                            // println!("typechecking {:?}.", decl.identifier);
                            //let value_type = self.type_check_ast(&mut decl.value);
                            // println!("... got {:?}.", value_type);
                            // println!("got value type as {:?}.", value_type);

                            match value_type {
                                Some(t) => decl.typ.primative = t.primative,
                                None => panic!()
                            };
                        }
                    }
                }
            },
            None => {}
        }
        

        match &mut decl.typ {
            Type{primative: Primative::FN(func), ..} => {
                match decl.identifier {
                    Token::IDENTIFIER(identifier) => {
                        func.anonymous_name = identifier.to_string()
                    },
                    _ => panic!()
                }
            },
            Type{primative: Primative::TYPE(typeType), ..} => {
                match decl.identifier {
                    Token::IDENTIFIER(identifier) => {
                        typeType.anonymous_name = identifier.to_string()
                    },
                    _ => panic!()
                }
            }
            _ => {}
        };

        println!("type check decl! {:?} {:?}.", decl.identifier.to_owned(), decl.typ);

        match decl.identifier {
            Token::IDENTIFIER(identifier) => self.sym_table.add(identifier.to_string(), decl.typ.to_owned()),
            _ => panic!()
        }

        None
    }

    fn type_check_func(&mut self, func: &mut ParsedFn) -> Option<Type> {
        // todo
        self.type_check_ast(&mut func.body);
        match &func.typ.primative {
            Primative::FN(func) => {
                match &func.return_type {
                    Some(return_type) => return Some(*return_type.to_owned()),
                    None => return None
                }
            }
            _ => panic!()
        }
    }

    fn type_check_num(&self, num: &f32) -> Option<Type> {
        // todo
        Some(Type { mutability: Mutability::CONSTANT, primative: Primative::I32, reference: false })
    }

    fn type_check_identifier(&self, identifier: &std::string::String) -> Option<Type> {
        // todo
        // Some(Type { mutability: Mutability::CONSTANT, primative: Primative::I32, reference: false })
        let t = self.sym_table.get(identifier.to_string()).unwrap();
        Some(t.to_owned())
    }

    fn type_check_string(&self, s: &std::string::String) -> Option<Type> {
        // todo
        // todo primitive strings!
        Some(Type { mutability: Mutability::CONSTANT, primative: Primative::STRING, reference: false })
    }

    fn type_check_left_unary(&mut self, left_unary: &mut LeftUnary) -> Option<Type> {


        match left_unary {
            LeftUnary::TAKE_REFERENCE(take_reference) => {
                let rhs = self.type_check_ast(&mut take_reference.rhs);
                take_reference.rhs_type = rhs.unwrap(); // todo this is bad
            }
        }

        None // todo
    }
    
    fn type_check_call(&mut self, call: &mut Call) -> Option<Type> {

        for arg in call.args.iter_mut() {
            self.type_check_ast(arg);
        }

        None
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