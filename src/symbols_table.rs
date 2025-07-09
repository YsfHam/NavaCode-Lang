use std::collections::HashMap;

use crate::types::Type;

pub struct SymbolsTable {
    scopes: Vec<Scope>,
    functions: HashMap<String, FunctionSymbol>,
}

impl SymbolsTable {
    pub fn new() -> Self {
        SymbolsTable {
            scopes: vec![Scope::new_global()],
            functions: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self, current_scope_id: ScopeId) -> ScopeId {
        let new_scope = Scope::new(current_scope_id);
        self.scopes.push(new_scope);
        ScopeId(self.scopes.len() - 1)
    }

    pub fn exit_scope(&mut self, current_scope_id: ScopeId) -> ScopeId {
        self.scopes[current_scope_id.0].parent.expect("Cannot exit global scope")
    }

    pub fn define_variable(&mut self, symbol: VariableSymbol, current_scope_id: ScopeId) {
        let scope = &mut self.scopes[current_scope_id.0];
        scope.add_variable(symbol);
    }

    pub fn define_function(&mut self, symbol: FunctionSymbol) {
        self.functions.insert(symbol.identifier.clone(), symbol);
    }

    pub fn lookup_variable_in_scope_only(&self, identifier: &str, current_scope_id: ScopeId) -> Option<&VariableSymbol> {
        let scope = &self.scopes[current_scope_id.0];
        scope.lookup(identifier)
    }

    pub fn lookup_function(&self, identifier: &str) -> Option<&FunctionSymbol> {
        self.functions.get(identifier)
    }

    pub fn lookup_variable(&self, identifier: &str, current_scope_id: ScopeId) -> Option<&VariableSymbol> {
        let mut current_lookup_scope_id = Some(current_scope_id);

        while let Some(scope_id) = current_lookup_scope_id {
            let scope = &self.scopes[scope_id.0];
            if let Some(symbol) = scope.lookup(identifier) {
                return Some(symbol);
            }
            current_lookup_scope_id = scope.parent;
        }

        None
    }
}


pub struct VariableSymbol {
    pub identifier: String,
    pub sym_type: Type,
}

pub struct FunctionSymbol {
    pub identifier: String,
    pub parameters: Vec<String>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeId(pub usize);

struct Scope {
    variables: HashMap<String, VariableSymbol>,
    parent: Option<ScopeId>,
}

impl Scope {
    fn new(parent: ScopeId) -> Self {
        Scope {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    fn new_global() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
        }
    }

    fn add_variable(&mut self, symbol: VariableSymbol) {
        self.variables.insert(symbol.identifier.clone(), symbol);
    }

    pub fn lookup(&self, identifier: &str) -> Option<&VariableSymbol> {
        self.variables.get(identifier)
    }
}