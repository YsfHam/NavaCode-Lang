use crate::{ast::Ast, diagnostic::Diagnostic, lexer::Lexer, parser::Parser};

pub struct CompilationUnit {
    pub ast: Ast
}

pub struct SourceCode {
    code: String,
}

impl SourceCode {
    pub fn from_string(code: String) -> Self {
        SourceCode { code }
    }

    pub fn as_str(&self) -> &str {
        &self.code
    }
}

pub struct Compiler {
    _private: (),
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { _private: () }
    }

    pub fn compile(&self, source_code: SourceCode) -> Result<CompilationUnit, Diagnostic> {
        let lexer = Lexer::new(source_code.as_str());

        let mut parser = Parser::new(lexer);
        
        println!("Parsing tokens...");

        Ok(CompilationUnit {
            ast: parser.parse()?
        })
    }
}