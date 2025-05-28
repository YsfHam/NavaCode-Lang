use navacodelang::{ast::AstExplorer, compiler::{Compiler, SourceCode}, interpreter::Interpreter, utils::AstDebugPrinter};


fn main() {

    let input = r#"
        let x be 20
        let y be 70
        let z be 0
        if x == y then
            set z to 1
        else if x > y then
            set z to 2
            set y to 7
        else
            set x to 0
            set y to 0
            set z to 99999
        end

        let u be 10
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
            Interpreter::interpret(&compilation_unit.ast);
        },
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
        }
    }
    println!("Compilation finished.");
}
