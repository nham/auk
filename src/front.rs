use std::collections::HashSet;

use libsyn;

pub enum Expression_<N> {
    Empty,
    Terminal(char),
    AnyTerminal, // any terminal
    TerminalString(String), // not in Ford's paper. more compact than a Seq of terminals
    Nonterminal(N),
    Seq(Vec<Expression_<N>>),
    Alt(Vec<Expression_<N>>),
    Optional(Box<Expression_<N>>), // ?
    ZeroOrMore(Box<Expression_<N>>), // *
    OneOrMore(Box<Expression_<N>>), // +
    PosLookahead(Box<Expression_<N>>), // & predicate in Ford's paper
    NegLookahead(Box<Expression_<N>>), // ! predicate in Ford's paper
    Class(HashSet<char>),
}

pub type Expression = Expression_<libsyn::Ident>;

pub struct Grammar {
    pub name: libsyn::Ident,
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub name: libsyn::Ident,
    pub expr: Box<Expression>
}

pub fn parse_grammar(parser: &mut libsyn::Parser) -> Grammar {
    if !consume_grammar_keyword(parser) {
        let tok = parser.this_token_to_string();
        let span = parser.span;
        parser.span_fatal(span,
            format!("Expected grammar declaration of the form `grammar <name> \
                    {{...}}` but found `{}`", tok).as_slice());
    }

    let name = parser.parse_ident();
    parser.expect(&libsyn::LBRACE);
    let mut v = vec!();
    v.push( parse_rule(parser) );
    //thing goes here
    parser.expect(&libsyn::RBRACE);
    Grammar { name: name, rules: v }
}

fn consume_grammar_keyword(parser: &mut libsyn::Parser) -> bool {
    // the second value attached to IDENT is the "is_mod_name" flag
    match parser.token {
        libsyn::IDENT(ident, false) if "grammar" == ident.as_str() => {
            parser.bump();
            true
        },
        _ => false,
    }
}

fn parse_rule(parser: &mut libsyn::Parser) -> Rule {
    let name = parser.parse_ident();
    parser.expect(&libsyn::EQ);
    Rule { name: name, expr: box parse_rule_expr(parser) }
}

fn parse_rule_expr(parser: &mut libsyn::Parser) -> Expression {
    match parser.token {
        libsyn::BINOP(libsyn::AND) => {
            parser.bump();
            return PosLookahead(box parse_rule_expr(parser));
        },
        libsyn::NOT => {
            parser.bump();
            return NegLookahead(box parse_rule_expr(parser));
        },
        libsyn::LIT_CHAR(name) => {
            parser.bump();
            return Terminal( libsyn::get_name(name).get().char_at(0) );
        },
        libsyn::LIT_STR(name) => {
            parser.bump();
            return TerminalString( libsyn::get_name(name).get().to_string() );
        },
        _ => {
            fail!("Unimplemented");
        },
    }
}
