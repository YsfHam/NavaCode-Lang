use navacodelang::{ast::AstExplorer, compiler::{Compiler, SourceCode}, interpreter::Interpreter, utils::AstDebugPrinter};


fn main() {

    let source_code = SourceCode::from_file("testing.nvc").expect("Cannot read source file");
    println!("Starting compilation...");
    let compiler = Compiler::new();
    let compilation_result = compiler.compile(&source_code);
    match compilation_result {
        Ok(compilation_unit) => {
            println!("Compilation successful!");
            AstDebugPrinter::new().explore_ast(&compilation_unit.ast);
            println!("Running code...");
            Interpreter::interpret(&compilation_unit.ast);
        },
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
        }
    }
    println!("Compilation finished.");
}
