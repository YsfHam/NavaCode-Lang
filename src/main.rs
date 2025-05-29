use navacodelang::{ast::AstExplorer, compiler::{Compiler, SourceCode}, interpreter::Interpreter, utils::AstDebugPrinter};


fn main() {

    let input = r#"
        let n be 29
        let is_prime be false

        let i be 2
        let is_searching be true
        while is_searching do
            if i >= n then
                set is_prime to true
                set is_searching to false
            else if n % i == 0 then
                set is_searching to false
            else
                set i to i + 1
            end
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
