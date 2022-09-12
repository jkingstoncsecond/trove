use std::io::Write;

use generator::Generator;

mod lex;
mod parse;
mod generator;

fn main() {

    std::env::set_var("RUST_BACKTRACE", "1");

    let mut lexer = lex::Lexer::new();
    lexer.lex(Box::new(std::string::String::from("abc(1)")));

    let mut parser = parse::Parser::new(&mut lexer.tokens);
    let ast = parser.parse();


    let generator = generator::CGenerator::new(ast);
    let code = generator.generate();

    let mut f = std::fs::File::create("/Users/james/dev/trove/build/build.c").expect("Unable to create file");
    f.write_all(code.as_bytes()).expect("Unable to write code out as bytes");
    
    std::process::Command::new("clang")
        .arg("/Users/james/dev/trove/build/build.c")
        .arg("-o")
        .arg("/Users/james/dev/trove/build/build")
        .output()
        .expect("Unable to compile code");

    unsafe {
        println!("creating context.");
        let context = llvm_sys::core::LLVMContextCreate();
        let module = llvm_sys::core::LLVMModuleCreateWithName(b"my_module\0".as_ptr() as *const _);
        let builder = llvm_sys::core::LLVMCreateBuilderInContext(context);

        // Get the type signature for void nop(void);
        // Then create it in our module.
        let void = llvm_sys::core::LLVMVoidTypeInContext(context);
        let function_type = llvm_sys::core::LLVMFunctionType(void, std::ptr::null_mut(), 0, 0);
        let function =
        llvm_sys::core::LLVMAddFunction(module, b"nop\0".as_ptr() as *const _, function_type);

        // Create a basic block in the function and set our builder to generate
        // code in it.
        let bb = llvm_sys::core::LLVMAppendBasicBlockInContext(
            context,
            function,
            b"entry\0".as_ptr() as *const _,
        );
        llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

        // Emit a `ret void` into the function
        llvm_sys::core::LLVMBuildRetVoid(builder);

        // Dump the module as IR to stdout.
        llvm_sys::core::LLVMDumpModule(module);

        // Clean up. Values created in the context mostly get cleaned up there.
        llvm_sys::core::LLVMDisposeBuilder(builder);
        llvm_sys::core::LLVMDisposeModule(module);
        llvm_sys::core::LLVMContextDispose(context);


    }
}
