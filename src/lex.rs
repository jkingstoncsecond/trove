
#[derive(Debug)]
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

pub struct Lexer {
    pub current: usize
}

impl Lexer {
    pub fn lex(&mut self, program: &std::string::String) -> Box<Vec<Token>>{

        let mut tokens = Box::new(vec!());

        println!("lexing {}.", &program);

        while !self.end(&program) {
 
            println!("counter {}.", self.current);


            match program.chars().nth(self.current).unwrap() {
                '+' => tokens.push(Token::PLUS),
                '-' => tokens.push(Token::MINUS),
                '*' => tokens.push(Token::STAR),
                '/' => tokens.push(Token::DIV),
                '{' => tokens.push(Token::LCURLY),
                '}' => tokens.push(Token::RCURLY),
                '(' => tokens.push(Token::LPAREN),
                ')' => tokens.push(Token::RPAREN),
                '[' => tokens.push(Token::LBRACKET),
                ']' => tokens.push(Token::RBRACKET),
                ',' => tokens.push(Token::COMMA),
                ':' => tokens.push(Token::COLON),
                ';' => tokens.push(Token::SEMICOLON),
                _ => {
                    // todo do identifier
                    self.other(&mut tokens, &program);
                }
            }

            self.current+=1;

        }

        return tokens;
    }

    fn end(&self, program: &std::string::String) -> bool {
        return self.current >= program.chars().count();
    }

    fn other(&mut self, tokens: &mut Box<Vec<Token>>, program: &std::string::String) {
        let c = program.chars().nth(self.current).unwrap();
        if c.is_digit(10) {
            self.number(tokens, &program);
        }else if c.is_alphabetic(){
            self.identifier(tokens,&program);
        }
    }

    fn number(&mut self, tokens: &mut Box<Vec<Token>>, program: &std::string::String){
        let mut s = std::string::String::from("");
        while !self.end(program) && program.chars().nth(self.current).unwrap().is_digit(10){
            s.push(program.chars().nth(self.current).unwrap());
            self.current+=1;
        }
        tokens.push(Token::NUMBER(s));
    }

    fn identifier(&mut self, tokens: &mut Box<Vec<Token>>, program: &std::string::String){
        let mut s = std::string::String::from("");
        while !self.end(program) && program.chars().nth(self.current).unwrap().is_alphabetic(){
            s.push(program.chars().nth(self.current).unwrap());
            self.current+=1;
        }
        tokens.push(Token::IDENTIFIER(s));
    }
}