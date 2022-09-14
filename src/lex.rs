
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

    EQUAL,

    NUMBER(std::string::String),
    STRING(std::string::String),
    IDENTIFIER(std::string::String),

    U32,
    I32,
    BOOL,
    FN

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
                '\n' => {},
                '\t' => {},
                '\r' => {},
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
                '=' => self.tokens.push(Token::EQUAL),
                'i' => {
                    if self.is_keyword("i32".to_string()) {
                        self.tokens.push(Token::I32);
                        self.current+=2; // its only 2 because we + 1 later
                    }
                },
                'u' => {
                    if self.is_keyword("u32".to_string()) {
                        self.tokens.push(Token::U32);
                        self.current+=2; // its only 2 because we + 1 later
                    }
                }
                ' ' => {},
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

    fn is_keyword(&self, keyword: std::string::String) -> bool {
        let mut matched = true;
        for i in 0..keyword.chars().count(){
            if self.program.chars().nth(self.current+i).unwrap() != 
                keyword.chars().nth(i).unwrap() {
                    matched = false;
                }
        }
        matched
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
        }else if c.eq_ignore_ascii_case(&'"') || c.eq_ignore_ascii_case(&'\''){
            self.string();
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

    fn string(&mut self){
        let first_char = self.program.chars().nth(self.current).unwrap();
        self.current+=1;
        let mut s = std::string::String::from("");
        while !self.end() && 
        !self.program.chars().nth(self.current).unwrap().eq_ignore_ascii_case(&first_char){
            s.push(self.program.chars().nth(self.current).unwrap());
            self.current+=1;
        }
        self.current+=1;
        self.tokens.push(Token::STRING(s));
    }
}