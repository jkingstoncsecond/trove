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
pub struct Binary<'a>{
    pub left: Box<ParsedAST<'a>>,
    pub op: &'a Token, // todo this should probably be a ref
    pub right: Box<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Group<'a>{
    pub expression: Box<ParsedAST<'a>>
}

#[derive(Debug)]
pub struct Call<'a>{
    pub callee: Box<ParsedAST<'a>>,
    pub args: Option<Box<ParsedAST<'a>>>
}

#[derive(Debug)]
pub struct Fn<'a>{
    pub body: Box<ParsedAST<'a>>
}


#[derive(Debug)]
pub enum ParsedAST<'a> {
    PROGRAM(Program<'a>),
    STMT(Box<ParsedAST<'a>>),
    BLOCK(Block<'a>),
    IF(If<'a>),
    DECL(Decl<'a>),
    IDENTIFIER(std::string::String),
    STRING(std::string::String),
    FN(Fn<'a>),
    NUMBER(f32),
    BINARY(Binary<'a>),
    GROUP(Group<'a>),
    CALL(Call<'a>),
    STRUCT_TYPES_LIST(StructTypesList<'a>),
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
     

        match self.consume(current) { 

            // todo these are mut
            Token::U32 => Type{mutability, primative: Primative::U32},
            Token::I32 => Type{mutability, primative: Primative::I32},
            Token::BOOL => Type{mutability, primative: Primative::BOOL},
            Token::FN => Type{mutability, primative: Primative::FN(FnType{args: vec![], anonymous_name: "anon".to_string()})},
            Token::TYPE => Type{mutability, primative: Primative::TYPE(TypeType{anonymous_name: "anon".to_string()})},
            _ => panic!()
        }

    }

    fn statement(&self, current: &mut usize) -> ParsedAST {
        match self.peek(&current) {
            Token::LCURLY => self.block(current),
            Token::IF => self.if_stmt(current),
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
                    Token::FN => {
                        let identifier = self.consume(current);
                        let typ = self.parse_type(current);

                        let mut value: Option<Box<ParsedAST>> = None;

                        // constant
                        match self.peek(current) {
                            Token::LCURLY => {
                                value = Some(Box::new(self.block(current)));
                            },
                            _ => {}
                        };

                        return ParsedAST::DECL(Decl{identifier, typ, requires_infering: false, value})
                    },
                    // todo we need to match for a type here instead of identifier
                    Token::U32 | Token::I32 | Token::BOOL => {

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

                        return ParsedAST::DECL(Decl{identifier, typ, requires_infering: false, value})
                    },
                    Token::EQUAL => {
                        let identifier = self.consume(current);
                        let typ = Type{mutability: Mutability::CONSTANT, primative: Primative::NONE};
                        self.consume(current); // consume the =
                        let value = self.expression(current);
                        return ParsedAST::DECL(Decl{identifier, typ, requires_infering: true, value: Some(Box::new(value))})

                    },
                    _ => return self.assign(current)
                }
            },
            _ => return self.assign(current)
        }
    }

    fn assign(&self, current: &mut usize) -> ParsedAST {
        self.plus_or_minus(current)
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

                                // todo SUPPORT NO ARGS
                                if !self.expecting(Token::RPAREN, current){
                                    let expr = self.expression(current);
                                    self.consume(current);
                                    return ParsedAST::CALL(Call{callee: Box::new(higher_presedence), args: Some(Box::new(expr))})
                                }else{
                                    self.consume(current);
                                    return ParsedAST::CALL(Call{callee: Box::new(higher_presedence), args: None})
                                }
                                
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
        self.single(current)
    }

    fn single(&self, current: &mut usize) -> ParsedAST {
        match self.peek(current) {
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
}