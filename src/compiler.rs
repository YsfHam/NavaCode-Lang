use std::{fs, io, path::Path};

use crate::{ast::Ast, diagnostic::Diagnostics, lexer::Lexer, parser::Parser, resolver::Resolver, symbols_table::SymbolsTable};

pub struct CompilationUnit {
    pub ast: Ast,
    pub symbols_table: SymbolsTable,
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

    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self::from_string(fs::read_to_string(path)?))
    }
}

pub struct Compiler {
    _private: (),
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { _private: () }
    }

    pub fn compile(&self, source_code: &SourceCode) -> Result<CompilationUnit, Diagnostics> {
        let lexer = Lexer::new(source_code.as_str());

        let parser = Parser::new(lexer);
        
        println!("Parsing tokens...");
        let ast = parser.parse()?;

        println!("Resolving symbols...");
        let symbols_table = Resolver::new().resolve(&ast)?;

        Ok(CompilationUnit {
            ast,
            symbols_table,
        })
    }
}