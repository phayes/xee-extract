#[derive(Debug)]
pub(crate) struct Scope<N: Eq + Clone> {
    names: Vec<N>,
}

impl<N: Eq + Clone> Scope<N> {
    fn new() -> Self {
        Self { names: Vec::new() }
    }

    fn get(&self, name: &N) -> Option<usize> {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return Some(i);
            }
        }
        None
    }

    fn known_name(&self, name: &N) -> bool {
        self.names.iter().any(|n| n == name)
    }
}

#[derive(Debug, Default)]
pub struct Scopes<N: Eq + Clone> {
    scopes: Vec<Scope<N>>,
}

impl<N: Eq + Clone> Scopes<N> {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub(crate) fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub(crate) fn push_name(&mut self, name: &N) {
        self.scopes.last_mut().unwrap().names.push(name.clone());
    }

    pub(crate) fn pop_name(&mut self) {
        self.scopes.last_mut().unwrap().names.pop();
    }

    pub(crate) fn get(&self, name: &N) -> Option<usize> {
        self.scopes.last().unwrap().get(name)
    }

    pub(crate) fn is_closed_over_name(&self, name: &N) -> bool {
        let mut scopes = self.scopes.iter();
        scopes.next();
        scopes.any(|s| s.known_name(name))
    }
}
