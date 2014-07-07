#[deriving(Show)]
enum PEGExpr<T, N> {
    Empty,
    Terminal(T),
    NonTerminal(N),
    Dot,
    Class(Vec<T>), // could be eliminated. it's syntactic sugar for a bunch of Alts
    Concat(Box<PEGExpr<S>>, Box<PEGExpr<S>>),
    Alt(Box<PEGExpr<S>>, Box<PEGExpr<S>>),
    Question(Box<PEGExpr<S>>),
    Star(Box<PEGExpr<S>>),
    Plus(Box<PEGExpr<S>>),
    And(Box<PEGExpr<S>>),
    Not(Box<PEGExpr<S>>),
}

type Expr = PEGExpr<char, char>;

type TermStr = Vec<char>,

fn parse<'a, T, N>(expr: PEGExpr<T,N>, input: &'a mut Vec<T>) 
    -> (uint, Option<&'a mut Vec<T>>) {

    match expr {
        Empty => (1, vec!()),
        Terminal(t) =>
    }
}

fn main() {
    let e = Concat(box Terminal('a'), box Terminal('b'));
    println!("{}", e);
}
