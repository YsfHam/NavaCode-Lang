use navacodelang::{ast::AstExplorer, compiler::{Compiler, SourceCode}, interpreter::Interpreter, utils::AstDebugPrinter};


fn main() {

    let input = r#"
let n be 5
let factorial be 1
let i be 1

set factorial to factorial * i
set i to i + 1

set factorial to factorial * i
set i to i + 1

set factorial to factorial * i
set i to i + 1

set factorial to factorial * i
set i to i + 1

set factorial to factorial * i
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
            let mut interpreter = Interpreter::new();
            interpreter.explore_ast(&compilation_unit.ast);
            interpreter.display_state();

        },
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
        }
    }
    println!("Compilation finished.");
}
