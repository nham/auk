use std::collections::HashSet;

// T = terminals, N = non-terminals
pub enum PEGExpr<T, N> {
    Empty,
    Terminal(T),
    Nonterminal(N),
    Dot,
    Class(HashSet<T>),
    Seq(Box<PEGExpr<T, N>>, Box<PEGExpr<T, N>>),
    Alt(Box<PEGExpr<T, N>>, Box<PEGExpr<T, N>>),
    Question(Box<PEGExpr<T, N>>),
    Star(Box<PEGExpr<T, N>>),
    Plus(Box<PEGExpr<T, N>>),
    PosLookahead(Box<PEGExpr<T, N>>), // & predicate in Ford's paper
    NegLookahead(Box<PEGExpr<T, N>>), // ! predicate in Ford's paper
}
