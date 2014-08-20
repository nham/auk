use std::collections::HashMap;

use expr::Expression;
use libsyn;
use front;

struct Grammar {
    name: libsyn::Ident,
    rules: HashMap<libsyn::Ident, Expression>,
}

pub fn convert(g: front::Grammar) -> Option<Grammar> {
    let mut map = HashMap::new();
    for front::Rule{name: name, expr: expr} in g.rules.move_iter() {
        map.insert(name, *expr);
    }
    Some( Grammar { name: g.name, rules: map })
}
