use std::collections::HashMap;

use libsyn;
use front::{mod, Expression};

pub type Grammar = Grammar_<libsyn::Ident>;

struct Grammar_<N> {
    pub name: libsyn::Ident,
    pub rules: HashMap<N, Expression>,
    pub start: N,
}

pub fn convert(g: front::Grammar) -> Option<Grammar> {
    let mut map = HashMap::new();
    let start = g.rules[0].name.clone();
    for front::Rule{name: name, expr: expr} in g.rules.move_iter() {
        map.insert(name, *expr);
    }
    Some( Grammar_ { name: g.name, rules: map, start: start })
}
