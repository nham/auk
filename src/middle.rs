use std::collections::HashMap;
use std::hash;

use libsyn;
use front::{mod, Expression};

pub type Grammar = Grammar_<libsyn::Ident>;

struct Grammar_<N> {
    pub rules: HashMap<N, Expression>,
    pub start: N,
}

impl<N> Grammar_<N>
    where N: Eq + hash::Hash {
    pub fn with_rules(map: HashMap<N, Expression>, start: N) -> Grammar_<N> {
        Grammar_ { rules: map, start: start }
    }
}

pub fn convert(g: front::Grammar) -> Option<Grammar> {
    let mut map = HashMap::new();
    let start = g.rules[0].name.clone();
    for front::Rule{name: name, expr: expr} in g.rules.move_iter() {
        map.insert(name, *expr);
    }
    Some( Grammar_::with_rules(map, start) )
}
