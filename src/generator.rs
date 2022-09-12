use crate::parse::ParsedAST;

pub trait Generator {
    fn generate<'a>(&self);
}


pub struct CGenerator<'a> {
    pub ast: &'a mut ParsedAST<'a>
}

impl Generator for CGenerator<'_> {
    fn generate(&self){
        println!("generating!");
        // self.generate_ast(self.ast)
    }
}

impl CGenerator<'_>{
    // pub fn new<'a>(ast: &'a mut ParsedAST) -> CGenerator<'a> {
    //     CGenerator {
    //        ast
    //     }
    // }

    // fn generate_ast<'a>(&self, ast: &'a mut ParsedAST){
    //     match ast {
    //         ParsedAST::PROGRAM(_) => {
    //             self.generate_program(ast);
    //         },
    //         _ => panic!()
    //     }
    // }

    // fn generate_program<'a>(&self, ast: &'a mut ParsedAST){
    //     println!("generating program!")
    // }
}