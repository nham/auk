use std::collections::HashMap;
use std::hash;

// T = terminals, N = non-terminals
#[deriving(Show)]
enum PEGExpr<T, N> {
    Empty,
    Terminal(T),
    Nonterminal(N),
    Dot,
    Class(Vec<T>), // could be eliminated. it's syntactic sugar for a bunch of Alts
    Seq(Box<PEGExpr<T, N>>, Box<PEGExpr<T, N>>),
    Alt(Box<PEGExpr<T, N>>, Box<PEGExpr<T, N>>),
    Question(Box<PEGExpr<T, N>>),
    Star(Box<PEGExpr<T, N>>),
    Plus(Box<PEGExpr<T, N>>),
    PosLookahead(Box<PEGExpr<T, N>>), // & predicate in Ford's paper
    NegLookahead(Box<PEGExpr<T, N>>), // ! predicate in Ford's paper
}

struct PEGGrammar<T, N> {
    rules: HashMap<N, PEGExpr<T, N>>,
}

type Expr = PEGExpr<char, char>;

type ParseResult<'a, T> = (uint, Option<&'a [T]>);

impl<T, N> PEGGrammar<T, N>
where T: Eq + Clone,
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
            _ => fail!("unimplemented"),
        }
    }
}

fn main() {
    let e: Expr = Seq(box Terminal('a'), box Terminal('b'));
    println!("{}", e);
}
