use navacodelang::{ast::AstExplorer, compiler::{Compiler, SourceCode}, interpreter::Interpreter, utils::AstDebugPrinter};


fn main() {

    let input = r#"
    define function add with a, b as
    return (a + b)
end

define function sub with a, b as
    return (a - b)
end

define function mul with a, b as
    return (a * b)
end

define function max with a, b as
    if a > b then
        return (a)
    else
        return (b)
    end
end

define function min with a, b as
    if a < b then
        return (a)
    else
        return (b)
    end
end

define function abs with x as
    if x < 0 then
        return (0 - x)
    else
        return (x)
    end
end

define function factorial with n as
    if n <= 1 then
        return (1)
    else
        return (n * factorial(n - 1))
    end
end

define function sum_to_n with n as
    if n <= 0 then
        return (0)
    else
        return (n + sum_to_n(n - 1))
    end
end

define function is_even with n as
    if n % 2 == 0 then
        return (1)
    else
        return (0)
    end
end

let a be add(5, 7)
let b be sub(10, 3)
let c be mul(a, b)
let d be max(a, b)
let e be min(a, b)
let f be abs(sub(b, a))
let g be factorial(5)
let h be sum_to_n(10)
let i be is_even(g)
let j be add(factorial(3), sum_to_n(4))
let k be mul(max(2, 8), min(3, 7))
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
