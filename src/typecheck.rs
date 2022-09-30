use std::hash::Hash;

use crate::{parse::{ParsedAST, Program, Decl, Binary, Block, Fn as ParsedFn, If, Assign, LeftUnary, Call, Directive, Number, For}, lex::Token};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fn{
    pub anonymous_name: std::string::String,
    pub args: Vec<Type>,
    pub return_type: Option<Box<Type>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeType{
    pub anonymous_name: std::string::String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependent{
    pub anonymous_name: std::string::String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primative{
    INCOMPLETE,
    U32,
    I32,
    F32,
    BOOL,
    STRING,
    FN(Fn),
    BLOCK,
    TYPE(TypeType),
    STRUCT(std::string::String),
    DEPENDENT(Dependent)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mutability {
    MUTABLE,
    CONSTANT
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub mutability: Mutability,
    pub primative: Primative,
    pub reference: bool
}

impl Type {
    // i.e. say we have &f32 and we want to call to_string() which takes a 
    // f32, we *can* cast the &f32 to a f32 by doing *f32 on the ptr.
    // if this returns true, we need to perform the dereferencing!

    // this fn returns whether a type is dependent on some value e.g.
    // type Foo { bar $T }
    // is dependent
    pub fn is_dependent(&self) -> bool{
        match &self.primative {
            Primative::INCOMPLETE => {return false;},
            Primative::U32 => {return false;},
            Primative::I32 => {return false;},
            Primative::F32 => {return false;},
            Primative::BOOL => {return false;},
            Primative::STRING => {return false;},
            Primative::FN(f) => {
                // todo check the fn
                let mut is_dependent = false;
                if f.return_type.is_some() {
                    if f.return_type.as_ref().unwrap().is_dependent() {
                        is_dependent = true;
                    }
                    for arg in f.args.iter() {
                        if arg.is_dependent() {
                            is_dependent = true;
                        }
                    }
                }
                return is_dependent;
            },
            Primative::BLOCK => {
                // todo
                return false;
            },
            Primative::TYPE(typ) => {
                let mut is_dependent = false;
                // todo
                // for t in typ.
                //     if arg.is_dependent() {
                //         is_dependent = true;
                //     }
                // }
                return is_dependent;
            },
            Primative::STRUCT(s) => {
                let mut is_dependent = false;
                // todo
                return is_dependent;
            },
            Primative::DEPENDENT(dependent) => {
                return true;
            }
        }
        false
    }

    pub fn size_in_bytes(&self) -> usize {
        // todo check if ref!
        match self.primative.to_owned() {
            Primative::U32 => 4,
            Primative::I32 => 4,
            Primative::F32 => 4,
            Primative::STRUCT(strct) => {
                // compute size of struct, we need the symbol table!
                let mut size = 0;
                size = 12;
                size
            },
            _ => panic!()
        }
    }

    fn can_be_autocast(&self, other: Type) -> bool {
        // todo we then will need to insert a 
        // todo this is only for reference casting
        if self.primative.eq(&other.primative) && (!self.reference && other.reference) {
            return true
        }
        false
    }

    fn shallow_equal(&self, other: Type) -> bool {
        // todo we then will need to insert a 
        // todo this is only for reference casting
        if self.primative.eq(&other.primative) {
            return true
        }
        false
    }
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
            ParsedAST::FOR(forr) => self.type_check_for(forr),
            ParsedAST::RET(ret) => self.type_check_ret(ret),
            ParsedAST::DECL(decl) => self.type_check_decl(decl),
            ParsedAST::ASSIGN(assign) => self.type_check_assign(assign),
            ParsedAST::FN(func) => self.type_check_func(func),
            ParsedAST::NUMBER(num) => self.type_check_num(num),
            ParsedAST::IDENTIFIER(identifier) => self.type_check_identifier(identifier),
            ParsedAST::STRING(s) => self.type_check_string(s),
            ParsedAST::LEFT_UNARY(left_unary) => self.type_check_left_unary(left_unary),//self.type_check_binary(binary),
            ParsedAST::BINARY(binary) => self.type_check_binary(binary),
            ParsedAST::CALL(call) => self.type_check_call(call), // todo
            ParsedAST::STRUCT_TYPES_LIST(s) => None, // todo
            ParsedAST::LHS_ACCESS(lhs_access) => None, // todo
            ParsedAST::GROUP(_) => None, // todo
            _ => panic!()
        }
    }
    
    fn type_check_directive(&mut self, directive: &mut Directive) -> Option<Type> {
        //self.type_check_ast(&mut directive.body.unwrap())
        None
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

    fn type_check_for(&mut self, forr: &mut For) -> Option<Type> {
        // println!("... if {:?}.", iff);
        // self.type_check_ast(&mut iff.condition);
        self.type_check_ast(&mut forr.body);
        // match &mut iff.else_body {
        //     Some(body) => {self.type_check_ast(body);},
        //     None => {}
        // };
        // todo 
        None
    }

    fn type_check_ret(&mut self, ret: &mut Option<Box<ParsedAST>>) -> Option<Type> {
        match ret {
            Some(val) => self.type_check_ast(val),
            None => None
        }
    }

    fn type_check_assign(&mut self, assign: &mut Assign) -> Option<Type> {
        let lhs_type = self.type_check_ast(&mut assign.lhs);
        let rhs_type = self.type_check_ast(&mut assign.rhs);

        println!("lhs type {:?} rhs type {:?}", lhs_type, rhs_type);

        if !lhs_type.unwrap().shallow_equal(rhs_type.unwrap()){


            // check if we can auto-cast
            // if lhs_type.as_ref().unwrap().can_be_autocast(rhs_type.as_ref().unwrap().clone()) {

            // }

        }
        // todo 
        // println!("ummm {:?}", lhs_type.unwrap().ref_can_be_assigned(rhs_type.unwrap()));
        // if lhs_type.unwrap().ref_can_be_assigned(rhs_type.unwrap()){
        //     println!("derefreencing!!!")

        //     // todo insert a dereference here somehow!

        // }
        None
    }

    fn type_check_decl(&mut self, decl: &mut Decl) -> Option<Type> {

        println!("doing decl... {:?}", decl.identifier);
        
        match &mut decl.value {
            Some(val) => {
                match val.as_mut() {
                    ParsedAST::FN(func) => {
                        let mut func_prim = &mut func.typ.primative;
                        match func_prim {
                            Primative::FN(prim_fn) => {
                                prim_fn.anonymous_name = decl.identifier.to_string();
                            },
                            _ => panic!()
                        }
                        decl.typ = func.typ.to_owned();
                        self.type_check_func(func);
                        // self.type_check_ast(&mut func.body);
                    },
                    _ => {
                        let value_type = self.type_check_ast(val);
                        if decl.requires_infering {
                            // println!("typechecking {:?}.", decl.identifier);
                            //let value_type = self.type_check_ast(&mut decl.value);
                            // println!("... got {:?}.", value_type);
                            // println!("got value type as {:?}.", value_type);

                            println!("... infering value_type {:?}", value_type);
                            match value_type {
                                Some(t) => {
                                    decl.typ.primative = t.primative;
                                    decl.typ.reference = t.reference;
                                },
                                None => panic!()
                            };
                        }
                        self.type_check_ast(val);
                    }
                }
            },
            None => {}
        }
        

        match &mut decl.typ {
            Type{primative: Primative::FN(func), ..} => {
                func.anonymous_name = decl.identifier.to_string();
            },
            Type{primative: Primative::TYPE(typeType), ..} => {
                typeType.anonymous_name = decl.identifier.to_string();
            }
            _ => {}
        };

        self.sym_table.add(decl.identifier.to_string(), decl.typ.to_owned());
        println!("decl {:?} is_dependent {:?}", decl.identifier, decl.typ.is_dependent());
        None
    }

    fn type_check_func(&mut self, func: &mut ParsedFn) -> Option<Type> {
        // todo
        for param in func.params.iter_mut(){
            self.type_check_decl(param);
        }

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

    fn type_check_num(&self, num: &Number) -> Option<Type> {
        match num {
            Number::INTEGER(_) => Some(Type { mutability: Mutability::CONSTANT, primative: Primative::I32, reference: false }),
            Number::FLOAT(_) => Some(Type { mutability: Mutability::CONSTANT, primative: Primative::F32, reference: false })
        }
    }

    fn type_check_identifier(&self, identifier: &std::string::String) -> Option<Type> {
        // todo

        println!("ummm {:?}", self.sym_table);
        println!("identifier {:?}", identifier);

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
                let mut cloned = take_reference.rhs_type.clone();
                if take_reference.rhs_type.reference {
                    cloned.reference = true;
                }else{
                    cloned.reference = true;
                }
                println!("..... rhs_type {:?}", take_reference.rhs_type);
                println!("..... cloned   {:?}", cloned);
                return Some(cloned);
            }
        }

        None // todo
    }
    
    fn type_check_call(&mut self, call: &mut Call) -> Option<Type> {
        

        for arg in call.args.iter_mut() {
            self.type_check_ast(arg);
        }

        let callee = call.callee.as_mut();
        match callee {
            ParsedAST::IDENTIFIER(identifier) => {
                // todo declare these as builtin!
                if !identifier.eq_ignore_ascii_case("println") {
                    let t = self.sym_table.get(identifier.to_string());
                    println!(".... ummm {:?}", identifier);
                    match t {
                        Some(typ) => {
                            let typtyp = typ;
                            match typtyp.primative.to_owned() {
                                Primative::FN(func) => {
                                    if func.return_type.is_some() {
                                        return Some(*func.return_type.unwrap());
                                    }
                                    return None;
                                },
                                _ => return None
                            }
                        },
                        None => panic!()
                    }
                }
            },
            _ => {}
        }

        None
    }

    fn type_check_binary(&mut self, binary: &mut Binary) -> Option<Type> {
        self.type_check_ast(&mut binary.right);
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