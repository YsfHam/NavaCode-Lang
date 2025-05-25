use navacodelang::{ast::AstExplorer, lexer, parser, utils::AstDebugPrinter};

fn main() {

    let input = r#"
    let x be --(1 + 2)
    set x to 5
    "#;
    println!("Starting lexer...");
    let mut parser = parser::Parser::new(lexer::Lexer::new(input));
    println!("Starting parsing...");
    let parsing_result = parser.parse();

    match parsing_result {
        Ok(ast) => {
            println!("Parsing successful!");
            let mut debug_printer = AstDebugPrinter::new();
            debug_printer.explore_ast(&ast);
        }
        Err(diagnostic) => {
            println!("Parsing failed with errors:");
            println!("{}", diagnostic);
        }
    }
}
