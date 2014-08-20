use std::collections::HashMap;

use peg::PEGGrammar;
use libsyn;
use front;

type Grammar = PEGGrammar<char, libsyn::Ident>;

pub fn convert(g: front::Grammar) -> Option<Grammar> {
    let mut map = HashMap::new();
    for front::Rule{name: name, expr: expr} in g.rules.move_iter() {
        map.insert(name, *expr);
    }
    Some( PEGGrammar::with_rules(map) )
}
