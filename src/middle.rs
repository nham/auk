use std::collections::HashMap;
use std::hash;

use libsyn;
use front::{mod, Expression};

pub type Grammar = Grammar_<libsyn::Ident>;

struct Grammar_<N> {
    rules: HashMap<N, Expression>,
}

impl<N> Grammar_<N>
    where N: Eq + hash::Hash {
    pub fn new() -> Grammar_<N> {
        Grammar_ { rules: HashMap::new() }
    }

    pub fn with_rules(map: HashMap<N, Expression>) -> Grammar_<N> {
        Grammar_ { rules: map }
    }
}

pub fn convert(g: front::Grammar) -> Option<Grammar> {
    let mut map = HashMap::new();
    for front::Rule{name: name, expr: expr} in g.rules.move_iter() {
        map.insert(name, *expr);
    }
    Some( Grammar_::with_rules(map) )
}
