extern crate syntax;

use std::collections::{HashMap};
use std::hash;

use expr::{PEGExpr, Empty, Terminal, Nonterminal, Dot, Seq, Alt, Class,
           Question, Star, Plus, PosLookahead, NegLookahead};

mod expr;
mod libsyn;

struct PEGGrammar<T, N> {
    rules: HashMap<N, PEGExpr<T, N>>,
}

type ParseResult<'a, T> = (uint, Option<&'a [T]>);

impl<T, N> PEGGrammar<T, N>
    where N: Eq + hash::Hash {
    fn new() -> PEGGrammar<T, N> {
        PEGGrammar { rules: HashMap::new() }
    }
}

impl<T, N> PEGGrammar<T, N>
where T: Eq + Clone + hash::Hash,
      N: Eq + hash::Hash {

    // In contrast to Ford's paper, we return the unconsumed input as second
    // component, not the consumed input.
    fn parse<'a>(&self, expr: &PEGExpr<T,N>, input: &'a [T]) -> ParseResult<'a, T> {
        match *expr {
            Empty => (1, Some(input)),
            Terminal(ref t) =>
                match input {
                    [ref a, ..rest] if a == t => (1, Some(rest)),
                    _ => (1, None),
                },
            Nonterminal(ref n) => {
                // we assume that we never try to parse an invalid nonterminal
                // will fail otherwise
                let (n, rem) = self.parse(self.rules.find(n).unwrap(), input);
                (n + 1, rem)
            },
            Dot =>
                match input {
                    [_, ..rest] => (1, Some(rest)),
                    _ => (1, None),
                },
            Seq(ref a, ref b) =>
                match self.parse(&**a, input) {
                    (i, None) => (i + 1, None),
                    (i, Some(rem)) =>
                        match self.parse(&**b, rem) {
                            (j, rem2) => (i + j + 1, rem2),
                        },
                },
            Alt(ref a, ref b) =>
                match self.parse(&**a, input) {
                    (i, None) =>
                        match self.parse(&**b, input) {
                            (j, rem) => (i + j + 1, rem),
                        },
                    (i, rem) => (i + 1, rem),
                },
            Class(ref s) =>
                match input {
                    [ref a, ..rest] if s.contains(a) => (1, Some(rest)),
                    _ => (1, None),
                },
            Question(ref a) => // Question(e) = Alt(e, Empty)
                match self.parse(&**a, input) {
                    (i, None) => (i + 2, Some(input)),
                    (i, rem) => (i + 1, rem),
                },
            Star(ref a) =>
                match self.parse(&**a, input) {
                    (i, None) => (i + 1, Some(input)),
                    (i, Some(rem)) =>
                        match self.parse(expr, rem) {
                            (j, rem2) => (i + j + 1, rem2),
                        },
                },
            Plus(ref a) => // Plus(e) = Seq(e, Star(e))
                match self.parse(&**a, input) {
                    (i, None) => (i + 1, None),
                    (i, Some(rem)) =>
                        match self.parse(expr, rem) {
                            (j, None) => (i + j + 1, Some(rem)),
                            (j, rem2) => (i + j + 1, rem2),
                        },
                },
            PosLookahead(ref a) =>
                match self.parse(&**a, input) {
                    (i, None) => (i + 1, None),
                    (i, _) => (i + 1, Some(input)),
                },
            NegLookahead(ref a) =>
                match self.parse(&**a, input) {
                    (i, None) => (i + 1, Some(input)),
                    (i, _) => (i + 1, None),
                },
        }
    }
}

fn main() {
    //let e: Expr = Seq(box Terminal('a'), box Terminal('b'));
    //println!("{}", e);
    let g: PEGGrammar<char, String> = PEGGrammar::new();

    println!("{}", g.parse(&Empty, &['h', 'e', 'l', 'l', 'o']));
    println!("{}", g.parse(&Terminal('h'), &['h', 'e', 'l', 'l', 'o']));
    println!("{}", g.parse(&Terminal('z'), &['h', 'e', 'l', 'l', 'o']));
}
