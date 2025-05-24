use navacodelang::lexer;

fn main() {
    let input = r#"
        let x be 1 +-2
    "#;


    let lexer = lexer::Lexer::new(input);
    for token in lexer {
        println!("{:?}", token);
    }
}
