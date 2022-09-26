use crate::{parse::{ParsedAST, Program, Decl, Block, LeftUnary, Fn, Call, Assign, TakeReference}, lex::Token, typecheck::{Type, Mutability, Primative}};

static HEAP_ALLOC_THRESHOLD: usize = 4;

struct AnalysisContext{
    scope: u32
}

pub struct Analyser {
    
}

impl Analyser{
    pub fn analyse<'a>(&mut self, mut tmp: Box<ParsedAST<'a>>) -> Box<ParsedAST<'a>> {
        println!("-------------- analysing!");
        let mut analysis_context = AnalysisContext{scope: 0};
        self.analyse_ast(&mut analysis_context, &mut tmp);
        tmp
    }

    fn analyse_ast(&mut self, analysis_context: &mut AnalysisContext, ast: &mut ParsedAST) {
        // println!("... ast {:?}.", ast);
        match ast {
            ParsedAST::PROGRAM(program) => self.analyse_program(analysis_context, program),
            ParsedAST::BLOCK(block) => self.analyse_block(analysis_context, block),
            ParsedAST::STMT(stmt) => self.analyse_stmt(analysis_context, stmt),
            ParsedAST::DECL(decl) => self.analyse_decl(analysis_context, decl),
            ParsedAST::FN(func) => self.analyse_func(analysis_context, func),
            ParsedAST::RET(ret) => self.analyse_ret(analysis_context, ret),
            ParsedAST::LEFT_UNARY(left_unary) => 
                // umm how do we do this...
                self.analyse_left_unary(analysis_context, ast),
            _ => {}
        }
    }

    fn analyse_program(&mut self, analysis_context: &mut AnalysisContext, program: &mut Program) {
        for stmt in program.body.iter_mut(){
            self.analyse_ast(analysis_context, stmt);
        }
    }

    fn analyse_block(&mut self, analysis_context: &mut AnalysisContext, block: &mut Block) {
        for stmt in block.body.iter_mut(){
            self.analyse_ast(analysis_context, stmt);
        }
    }

    fn analyse_stmt(&mut self, analysis_context: &mut AnalysisContext, stmt: &mut Box<ParsedAST>) {
        self.analyse_ast(analysis_context, stmt);
    }

    fn analyse_decl(&mut self, analysis_context: &mut AnalysisContext, decl: &mut Decl) {

        println!("-----> analyse_decl");
        match &mut decl.value {
            Some(val) => {
                self.analyse_ast(analysis_context, val);
            }
            None => {}
        }
    }

    fn analyse_func(&mut self, analysis_context: &mut AnalysisContext, func: &mut Fn) {

        println!("-----> analyse_fn");
        self.analyse_ast(analysis_context, &mut func.body);
    }

    fn analyse_ret(&mut self, analysis_context: &mut AnalysisContext, ret: &mut Option<Box<ParsedAST>>) {
        match ret {
            Some(ast) => self.analyse_ast(analysis_context, ast),
            None => {}
        }
    }


    fn analyse_left_unary(&mut self, 
        analysis_context: &mut AnalysisContext, 
        left_unary_ast: &mut ParsedAST){
        // left_unary: &mut LeftUnary) {
        // todo
        match left_unary_ast {
            ParsedAST::LEFT_UNARY(left_unary) => {
                match left_unary {
                    LeftUnary::TAKE_REFERENCE(value) => {
                        // todo check if we are in a function
        
                        println!("analyse LeftUnary::TAKE_REFERENCE");
                        // todo do some escape analysis!
                        // todo get the size here!
                        // todo should we associate a type with every ast???
                        let rhs_type_size = 9;
                        if rhs_type_size >= HEAP_ALLOC_THRESHOLD {

                            // todo this is all wrong as the ast is a bit fudged here!
                            
                            println!("HEAP_ALLOC_THRESHOLD reached!");

                            value.is_heap_alloc = true;
                            // let heap_alloc_fn = ParsedAST::IDENTIFIER("malloc".to_string());
                            // let heap_alloc_args = ParsedAST::NUMBER(rhs_type_size as f32);
                            // let heap_alloc_call = ParsedAST::CALL(
                            //     Call{ callee: Box::new(heap_alloc_fn), args: vec![heap_alloc_args] });
                            // let identifier = Token::IDENTIFIER("tmp".to_string());
                            // let heap_alloc_decl = ParsedAST::DECL(
                            //     Decl{ 
                            //         identifier: "tmp".to_string(), 
                            //         typ: Type{ mutability: Mutability::CONSTANT, primative: Primative::I32, reference: true }, 
                            //         requires_infering: false, 
                            //         value: Some(Box::new(heap_alloc_call)) 
                            //     }
                            // );
                            // let heap_alloc_assign_lhs = ParsedAST::LEFT_UNARY(LeftUnary::TAKE_REFERENCE(
                            //     TakeReference{
                            //         rhs: Box::new(ParsedAST::IDENTIFIER("tmp".to_string())), 
                            //         rhs_type: Type { mutability: Mutability::CONSTANT, primative: Primative::I32, reference: true }
                            //     }
                            // ));
                            // // let rhs = value.rhs.as_mut();
                            // let heap_alloc_assign = ParsedAST::ASSIGN(
                            //     Assign{ lhs: Box::new(heap_alloc_assign_lhs), rhs: Box::new(ParsedAST::NUMBER(1.2)) });
                            //     // Assign{ lhs: Box::new(heap_alloc_assign_lhs), rhs: Box::new(rhs.) });
                            
                            
                            //     // todo set the left unary value?
                            // let block = ParsedAST::BLOCK(Block{
                            //     new_scope: false,
                            //     body: vec![
                            //         ParsedAST::STMT(Box::new(heap_alloc_decl)), 
                            //         ParsedAST::STMT(Box::new(heap_alloc_assign))
                            //     ]
                            // });
                            // *left_unary_ast = block;    
                        }
                    }
                }
            },
            _ => {}
        }
    }
}