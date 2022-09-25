use crate::lex::Token;
use crate::typecheck::{Type, Primative, Mutability, Fn as FnType, TypeType};

#[derive(Debug)]
pub struct Program<'a>{
    // todo this should probably be an array of Box<ParsedAST>
    pub body: Vec<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Block<'a>{
    pub body: Vec<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct StructTypesList<'a>{
    pub types: Vec<Decl<'a>>
}

#[derive(Debug)]
pub struct Decl<'a>{
    pub identifier: &'a Token,
    pub typ: Type,
    pub requires_infering: bool,
    pub value: Option<Box<ParsedAST<'a>>>,
}

#[derive(Debug)]
pub struct Assign<'a>{
    pub lhs: Box<ParsedAST<'a>>,
    pub rhs: Box<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct If<'a>{
    pub condition: Box<ParsedAST<'a>>,
    pub body: Box<ParsedAST<'a>>,
    pub else_body: Option<Box<ParsedAST<'a>>>
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub token: &'a Token
}

#[derive(Debug)]
pub struct TakeReference<'a> {
    pub rhs: Box<ParsedAST<'a>>,
    pub rhs_type: Type
}

#[derive(Debug)]
pub enum LeftUnary<'a>{
    TAKE_REFERENCE(TakeReference<'a>)
}

// todo turn this into an enum
#[derive(Debug)]
pub struct Binary<'a>{
    pub left: Box<ParsedAST<'a>>,
    pub op: &'a Token, // todo this should probably be a ref
    pub right: Box<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct LhsAccess<'a>{
    pub left: Box<ParsedAST<'a>>,
    // todo this should probably be an identifier?
    pub right: Box<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Group<'a>{
    pub expression: Box<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Call<'a>{
    pub callee: Box<ParsedAST<'a>>,
    pub args: Vec<ParsedAST<'a>>    
}

#[derive(Debug)]
pub struct Fn<'a>{
    pub typ: Type,
    pub params: Vec<Decl<'a>>,
    pub body: Box<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Directive<'a>{
    pub value: Token,
    pub args: Vec<ParsedAST<'a>>,
    pub body: Option<Box<ParsedAST<'a>>>
}

#[derive(Debug)]
pub enum ParsedAST<'a> {
    PROGRAM(Program<'a>),
    STMT(Box<ParsedAST<'a>>),
    BLOCK(Block<'a>),
    IF(If<'a>),
    RET(Option<Box<ParsedAST<'a>>>),
    DECL(Decl<'a>),
    ASSIGN(Assign<'a>),
    IDENTIFIER(std::string::String),
    STRING(std::string::String),
    FN(Fn<'a>),
    NUMBER(f32),
    LEFT_UNARY(LeftUnary<'a>),
    BINARY(Binary<'a>),
    GROUP(Group<'a>),
    CALL(Call<'a>),
    STRUCT_TYPES_LIST(StructTypesList<'a>),
    LHS_ACCESS(LhsAccess<'a>),
    DIRECTIVE(Directive<'a>),
}

pub struct Parser<'a>{
    pub tokens: &'a Box<Vec<Token>>
}
// todo should the ast be an enum?


impl Parser<'_> {

    pub fn new(tokens: &mut Box<Vec<Token>>) -> Parser {
        Parser {tokens}
    }

    pub fn parse(&mut self) -> Box<ParsedAST> {
        Box::new(self.parse_program())
    }

    fn parse_program(&mut self) -> ParsedAST{
        println!("parsing program!");

        let mut current: usize = 0;
        let mut body: Vec<ParsedAST> = vec!();
        
        while !self.end(&current){
            body.push(self.statement(&mut current));
            println!("done statement :)");
        }
        
        return ParsedAST::PROGRAM(Program{body: body});
    }

    fn parse_type(&self, current: &mut usize) -> Type {
        
        let mut mutability = Mutability::CONSTANT;

        //let mut mutability = Mutability::MUTABLE;

        // todo get this working in parse decl
        match self.peek(current) {
            Token::CONST => {
                self.consume(current);
                mutability = Mutability::CONSTANT;
            },
            Token::MUT => {
                self.consume(current);
                mutability = Mutability::MUTABLE;
            },
            _ => {}
        }

        // todo get this working in parse decl
        match self.peek(current) {
            Token::PUB => {
                self.consume(current);
            },
            Token::PRIV => {
                self.consume(current);
            },
            _ => {}
        }

        let mut reference = false;
        // todo get this working in parse decl
        match self.peek(current) {
            Token::AT => {
                self.consume(current);
                reference = true;
            },
            _ => {}
        }

        match self.peek(current) { 
            // todo these are mut
            // todo fix this VAR
            Token::VAR => {self.consume(current);return Type{reference, mutability, primative: Primative::INCOMPLETE}},
            Token::U32 => {self.consume(current);return Type{reference, mutability, primative: Primative::U32}},
            Token::I32 => {self.consume(current);return Type{reference, mutability, primative: Primative::I32}},
            Token::F32 => {self.consume(current);return Type{reference, mutability, primative: Primative::F32}},
            Token::BOOL => {self.consume(current);return Type{reference, mutability, primative: Primative::BOOL}},
            Token::FN => {
                self.consume(current);
                if self.expecting(Token::LPAREN, current) {
                    self.consume(current);
                    while !self.expecting(Token::RPAREN, current) {
                        self.consume(current);
                    }
                    self.consume(current);
                }
                return Type{reference, mutability, primative: Primative::FN(FnType{
                    args: vec![], anonymous_name: "anon".to_string(), return_type: None})}
            },
            Token::TYPE => {self.consume(current);return Type{reference, mutability, primative: Primative::TYPE(TypeType{anonymous_name: "anon".to_string()})}},
            Token::IDENTIFIER(identifier) => {self.consume(current);return Type{reference, mutability, primative: Primative::STRUCT(identifier.to_string())}},
            _ => Type{reference, mutability, primative: Primative::INCOMPLETE}
        }

    }

    fn statement(&self, current: &mut usize) -> ParsedAST {
        match self.peek(&current) {
            Token::LCURLY => self.block(current),
            Token::IF => self.if_stmt(current),
            Token::RET => self.ret(current),
            _ => ParsedAST::STMT(Box::new(self.expression(current)))
        }
    }

    fn expression(&self, current: &mut usize) -> ParsedAST {
        match self.peek(&current) {
            _ => self.comparison(current)
        }
    }

    fn comparison(&self, current: &mut usize) -> ParsedAST {
        self.decl_or_assign(current)
    }

    fn decl_or_assign(&self, current: &mut usize) -> ParsedAST {

        let first = self.peek(current);

        if self.end_ahead(current, 1) {
            return self.assign(current);
        }

        let second = self.peek_ahead(current, 1);

        match first {
            Token::IDENTIFIER(_) => {
                match second {
                    Token::TYPE => {
                        let identifier = self.consume(current);
                        let typ = self.parse_type(current);

                        let mut value: Option<Box<ParsedAST>> = None;

                        // constant
                        match self.peek(current) {
                            Token::LCURLY => {
                                value = Some(Box::new(self.struct_types_list(current)));
                            },
                            _ => {}
                        };

                        return ParsedAST::DECL(Decl{identifier, typ, requires_infering: false, value})
                    },
                    // todo
                    //  instead of parsing the fn type, just call self.fn() ?
                    Token::FN => {
                        let identifier = self.consume(current);
                        let typ = Type { mutability: Mutability::CONSTANT, primative: Primative::INCOMPLETE, reference: false };
                        let value = Some(Box::new(self.function(current)));

                        return ParsedAST::DECL(Decl{identifier, typ, requires_infering: true, value});

                        // let typ = self.parse_type(current);

                        // let mut value: Option<Box<ParsedAST>> = None;

                        // // constant
                        // match self.peek(current) {
                        //     Token::LCURLY => {
                        //         value = Some(Box::new(self.block(current)));
                        //     },
                        //     _ => {}
                        // };

                        // return ParsedAST::DECL(Decl{identifier, typ, requires_infering: false, value})
                    },
                    // todo we need to match for a type here instead of identifier
                    Token::AT |Token::VAR | Token::MUT | Token::CONST | Token::PUB | Token::PRIV | Token::U32 | Token::I32 | Token::F32 | Token::BOOL | Token::IDENTIFIER(_) => {

                        let identifier = self.consume(current);
                        let typ = self.parse_type(current);

                        let mut value: Option<Box<ParsedAST>> = None;

                        // constant
                        match self.peek(current) {
                            Token::EQUAL => {
                                self.consume(current); // consume the =
                                value = Some(Box::new(self.expression(current)))
                            },
                            _ => {}
                        };

                        match typ.primative {
                            Primative::INCOMPLETE => return ParsedAST::DECL(Decl{identifier, typ, requires_infering: true, value}),
                            _ => return ParsedAST::DECL(Decl{identifier, typ, requires_infering: false, value})
                        }
                    },
                    Token::EQUAL => {

                        // todo assign!

                        // let identifier = self.consume(current);
                        // let typ = Type{mutability: Mutability::CONSTANT, primative: Primative::NONE};
                        // self.consume(current); // consume the =
                        // let value = self.expression(current);
                        // return ParsedAST::DECL(Decl{identifier, typ, requires_infering: true, value: Some(Box::new(value))})
                        return self.assign(current);

                    },
                    _ => return self.assign(current)
                }
            },
            _ => return self.assign(current)
        }
    }

    fn assign(&self, current: &mut usize) -> ParsedAST {
        let higher_precedence = self.plus_or_minus(current);
        if self.expecting(Token::EQUAL, current) {
            self.consume(current);
            let rhs = self.plus_or_minus(current);
            return ParsedAST::ASSIGN(Assign{lhs: Box::new(higher_precedence), rhs: Box::new(rhs)})
        }
        higher_precedence
    }

    fn plus_or_minus(&self, current: &mut usize) -> ParsedAST {
        let higher_precedence = self.mul_or_div(current);

        if !self.end(current){
            match self.peek(current) {
                Token::PLUS | Token::MINUS => {
                    let token = self.consume(current);
                    let right = self.expression(current);
                    return ParsedAST::BINARY(Binary{
                        left: Box::new(higher_precedence),
                        op: token,
                        right: Box::new(right)
                    })
                },
                _ => return higher_precedence
            }
        }
        higher_precedence
    }

    fn mul_or_div(&self, current: &mut usize) -> ParsedAST {
        let higher_precedence = self.unary(current);

        if !self.end(current){
            match self.peek(current) {
                Token::STAR | Token::DIV => {
                    let token = self.consume(current);
                    let right = self.expression(current);
                    return ParsedAST::BINARY(Binary{
                        left: Box::new(higher_precedence),
                        op: token,
                        right: Box::new(right)
                    })
                },
                _ => return higher_precedence
            }
        }
        higher_precedence
    }

    fn unary(&self, current: &mut usize) -> ParsedAST {
        
        if self.expecting(Token::AT, current) {
            self.consume(current);
            let rhs = self.call(current);
            let rhs_type = Type{mutability: Mutability::CONSTANT, primative: Primative::INCOMPLETE, reference: false};
            return ParsedAST::LEFT_UNARY(LeftUnary::TAKE_REFERENCE(TakeReference{rhs: Box::new(rhs), rhs_type}))
        }

        self.call(current)
    }

    fn call(&self, current: &mut usize) -> ParsedAST {
        let higher_presedence = self.struct_access(current);
        if !self.end(current) {
            match self.peek_ahead(current, -1) {
                Token::IDENTIFIER(_) => {
                    if !self.end_ahead(current, 1){
                        // todo
                        // todo peak_ahead could fail :(
                        match self.peek(current) {
                            Token::LPAREN => {
                                self.consume(current);
                                let mut args: Vec<ParsedAST> = vec![];
                                while !self.expecting(Token::RPAREN, current){
                                    args.push(self.expression(current));
                                    if !self.expecting(Token::RPAREN, current) {
                                        self.consume(current); // consume the ,
                                    }
                                }
                                self.consume(current); // consume the )
                                return ParsedAST::CALL(Call{callee: Box::new(higher_presedence), args})
                            }
                            _ => return higher_presedence
                        }
                    }
                }
                _ => return higher_presedence
            }
        }

        higher_presedence
    }

    fn struct_access(&self, current: &mut usize) -> ParsedAST {

        let higher_precedence = self.single(current);

        match self.peek(current) {
            Token::DOT => {
                self.consume(current); // consume the dot
                let rhs = self.expression(current);
                return ParsedAST::LHS_ACCESS(LhsAccess{ left: Box::new(higher_precedence), right: Box::new(rhs) })
            },
            _ => {return higher_precedence}
        }
    }

    fn single(&self, current: &mut usize) -> ParsedAST {
        match self.peek(current) {
            Token::HASH => {
                self.consume(current);
                let value = self.consume(current);
                let mut args: Vec<ParsedAST> = vec![];
                if(self.expecting(Token::LPAREN, current)){
                    self.consume(current);

                    while !self.expecting(Token::RPAREN, current) {
                        args.push(self.expression(current));
                        if !self.expecting(Token::RPAREN, current){
                            self.consume(current); // consume the ,
                        }
                    }

                    self.consume(current); // consume the rparen
                }
                // todo directives don't need bodies!
                //let body = self.statement(current);
                ParsedAST::DIRECTIVE(Directive { value: value.clone(), args, body: None })
            },
            Token::TRUE => {
                self.consume(current);
                ParsedAST::NUMBER(1.0)
            },
            Token::FALSE => {
                self.consume(current);
                ParsedAST::NUMBER(0.0)
            },
            Token::IDENTIFIER(identifier) => {
                self.consume(current);
                ParsedAST::IDENTIFIER(identifier.to_string())
            },
            Token::STRING(string) => {
                self.consume(current);
                ParsedAST::STRING(string.to_string())
            },
            Token::NUMBER(number) => {
                self.consume(current);
                match number.parse::<f32>(){
                    Ok(num) => ParsedAST::NUMBER(num),
                    _ => panic!()
                }
            },
            Token::LPAREN => {
                self.consume(current);
                let expression = self.expression(current);
                self.consume(current);
                ParsedAST::GROUP(Group { expression: Box::new(expression) })
            },
            Token::LCURLY => {
                self.block(current)
            }
            _ => panic!()
        }
    }

    fn peek(&self, current: &usize) -> &Token {
        match self.tokens.get(*current) {
            std::option::Option::Some(t) => return t,
            _ => panic!("umm")
        }
    }

    fn peek_ahead(&self, current: &usize, amount: i32) -> &Token {
        match self.tokens.get((*current as i32 + amount) as usize) {
            std::option::Option::Some(t) => return t,
            _ => panic!("umm")
        }
    }

    fn end(&self, current: &usize) -> bool {
        *current >= self.tokens.len()
    }

    fn end_ahead(&self, current: &usize, amount: i32) -> bool {
        (*current as i32 + amount) as usize >= self.tokens.len()
    }

    fn expecting(&self, token: Token, current: &usize) -> bool {
        let next = self.peek(current);
        return token.eq(&next);
    }

    fn consume(&self, current: &mut usize) -> &Token {
        match self.tokens.get(*current) {
            std::option::Option::Some(t) => {
                *current+=1;
                return t;
            },
            _ => panic!("umm")
        }
    }

    fn block(&self, current: &mut usize) -> ParsedAST {
        self.consume(current);
        let mut body: Vec<ParsedAST> = vec!();
        while !self.end(current) && !self.expecting(Token::RCURLY, current) {
            body.push(self.statement(current));
        }
        self.consume(current);
        return ParsedAST::BLOCK(Block{body});
    }

    fn if_stmt(&self, current: &mut usize) -> ParsedAST {
        
        println!("doing if!");
        self.consume(current); // consume the if
        let condition = Box::new(self.expression(current));
        let body = Box::new(self.statement(current));
        let mut else_body: Option<Box<ParsedAST>> = None;

        if !self.end(current) && self.expecting(Token::ELSE, current) {
            self.consume(current); // consume the else
            else_body = Some(Box::new(self.statement(current)));
        }

        return ParsedAST::IF(If{condition, body, else_body });
    }

    fn ret(&self, current: &mut usize) -> ParsedAST {
        
        self.consume(current); // consume the return

        // todo how do we expect a value?

        return ParsedAST::RET(None);
    }

    fn struct_types_list(&self, current: &mut usize) -> ParsedAST {
        
        println!("doing struct types list!");
        self.consume(current); // consume the {

        let mut types: Vec<Decl> = vec!();
        while !self.end(current) && !self.expecting(Token::RCURLY, current) {
            match self.decl_or_assign(current) {
                ParsedAST::DECL(decl) => types.push(decl),
                _ => panic!("must be a decl in a struct type def!")
            }
        }

        self.consume(current); // consume the }

        return ParsedAST::STRUCT_TYPES_LIST(StructTypesList{types});
    }

    fn function(&self, current: &mut usize) -> ParsedAST {
        self.consume(current); // consume the fn 
        let mut params: Vec<Decl> = vec![];
        if self.expecting(Token::LPAREN, current) {
            self.consume(current);
            while !self.expecting(Token::RPAREN, current) {
                let decl = self.decl_or_assign(current);
                match decl {
                    ParsedAST::DECL(d) => params.push(d),
                    _ => panic!("must be decl!")
                }
                if !self.expecting(Token::RPAREN, current) {
                    self.consume(current);
                }
            }
            self.consume(current);
        }
        // todo parse the return type!

        // todo parse optional type :(

        let mut return_typ = self.parse_type(current);
        return_typ.mutability = Mutability::MUTABLE;
        let return_type = Some(Box::new(return_typ));
        // umm this should be expression but currently blocks are stmts :(
        let body = self.statement(current);
        let typ = Type{ mutability: Mutability::CONSTANT, primative: Primative::FN(FnType{
            anonymous_name: "anon".to_string(),
            args: vec![], // todo!
            return_type
        }), reference: false };
        ParsedAST::FN(Fn{params, typ, body: Box::new(body)})
    }
}