use std::collections::HashMap;

pub struct SymbolsTable {
    scopes: Vec<Scope>,
}

impl SymbolsTable {
    pub fn new() -> Self {
        SymbolsTable {
            scopes: vec![Scope::new_global()],
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

    pub fn define_symbol(&mut self, symbol: Symbol, current_scope_id: ScopeId) {
        let current_scope = &mut self.scopes[current_scope_id.0];
        current_scope.add_symbol(symbol)
    }

    pub fn lookup(&self, identifier: &str, current_scope_id: ScopeId) -> Option<&Symbol> {
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

    pub fn lookup_in_scope_only(&self, identifier: &str, current_scope_id: ScopeId) -> Option<&Symbol> {
        let scope = &self.scopes[current_scope_id.0];
        scope.lookup(identifier)
    }
}

pub struct Symbol {
    pub identifier: String,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeId(pub usize);

struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<ScopeId>,
}

impl Scope {
    fn new(parent: ScopeId) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent: Some(parent),
        }
    }

    fn new_global() -> Self {
        Scope {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.identifier.clone(), symbol);
    }

    pub fn lookup(&self, identifier: &str) -> Option<&Symbol> {
        self.symbols.get(identifier)
    }
}