use navacodelang::{ast::AstExplorer, compiler::{Compiler, SourceCode}, interpreter::Interpreter, utils::AstDebugPrinter};


fn main() {

    let input = r#"
        let x be false
        let y be 0
        if x then
            set y to 1
        else
            set y to 2
        end
    "#;

    println!("Starting compilation...");
    let compiler = Compiler::new();
    let compilation_result = 
        compiler.compile(
            SourceCode::from_string(input.to_string())
        );

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
