
#[derive(Debug, Eq, PartialEq)]
#[warn(dead_code)]
pub enum Token {
    END,

    PLUS,
    MINUS,
    STAR,
    DIV,

    LCURLY,
    RCURLY,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,

    COMMA,
    COLON,
    SEMICOLON,

    NUMBER(std::string::String),
    STRING,
    IDENTIFIER(std::string::String)
}

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "{}", self)
//     }
// }


pub struct Lexer {
    pub current: usize,
    pub program: Box<std::string::String>,
    // todo this is bad practice
    pub tokens: Box<Vec<Token>>
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            current: 0, 
            program: Box::new(std::string::String::from("")),
            tokens: Box::new(vec!())
        }
    }

    pub fn lex(&mut self, program: Box<std::string::String>) {//-> Box<Vec<Token>>{

        self.program = program;


        while !self.end() {
 

            match self.program.chars().nth(self.current).unwrap() {
                '+' => self.tokens.push(Token::PLUS),
                '-' => self.tokens.push(Token::MINUS),
                '*' => self.tokens.push(Token::STAR),
                '/' => self.tokens.push(Token::DIV),
                '{' => self.tokens.push(Token::LCURLY),
                '}' => self.tokens.push(Token::RCURLY),
                '(' => self.tokens.push(Token::LPAREN),
                ')' => self.tokens.push(Token::RPAREN),
                '[' => self.tokens.push(Token::LBRACKET),
                ']' => self.tokens.push(Token::RBRACKET),
                ',' => self.tokens.push(Token::COMMA),
                ':' => self.tokens.push(Token::COLON),
                ';' => self.tokens.push(Token::SEMICOLON),
                _ => {
                    // todo do identifier
                    self.other();
                    continue;
                }
            }

            self.current+=1;

        }


        for token in self.tokens.iter() {
            println!("token {:?}", token);
        }

    }

    fn end(&self) -> bool {
        return self.current >= self.program.chars().count();
    }

    fn other(&mut self) {
        let c = self.program.chars().nth(self.current).unwrap();
        if c.is_digit(10) {
            self.number();
        }else if c.is_alphabetic(){
            self.identifier();
        }
    }

    fn number(&mut self){
        let mut s = std::string::String::from("");
        while !self.end() && self.program.chars().nth(self.current).unwrap().is_digit(10){
            s.push(self.program.chars().nth(self.current).unwrap());
            self.current+=1;
        }
        self.tokens.push(Token::NUMBER(s));
    }

    fn identifier(&mut self){
        let mut s = std::string::String::from("");
        while !self.end() && self.program.chars().nth(self.current).unwrap().is_alphabetic(){
            s.push(self.program.chars().nth(self.current).unwrap());
            self.current+=1;
        }
        self.tokens.push(Token::IDENTIFIER(s));
    }
}