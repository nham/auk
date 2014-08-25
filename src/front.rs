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
    Class(String),
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
        _ => {
            let expr = parse_non_prefix(parser);
            match parser.token {
                libsyn::BINOP(libsyn::STAR) => {
                    parser.bump();
                    return ZeroOrMore(box expr);
                },
                libsyn::BINOP(libsyn::PLUS) => {
                    parser.bump();
                    return OneOrMore(box expr);
                },
                libsyn::QUESTION => {
                    parser.bump();
                    return Optional(box expr);
                },
                _ => return expr, // this is probably not right. need to check
                                  // if its the next rule or whatever
            }
        },
    }
}

// parse something that could be modified by one of the suffixes: ?, +, *
fn parse_non_prefix(parser: &mut libsyn::Parser) -> Expression {
    match parser.token {
        libsyn::LIT_CHAR(name) => {
            parser.bump();
            return Terminal( libsyn::get_name(name).get().char_at(0) );
        },
        libsyn::LIT_STR(name) => {
            parser.bump();
            return TerminalString( libsyn::get_name(name).get().to_string() );
        },
        libsyn::DOT => {
            parser.bump();
            return AnyTerminal;
        },
        libsyn::LBRACKET => {
            parser.bump();
            match parser.token {
                libsyn::LIT_STR(name) => {
                    parser.bump();
                    let s = libsyn::get_name(name).get().to_string();

                    match parser.token {
                        libsyn::RBRACKET => {
                            parser.bump();
                            return Class(s);
                        },
                        _ => fail!("Character class must end with ']'"),
                    }
                },
                _ => fail!("Character class has the form '[\"<chars>\"]'"),
            }

        }
        _ => {
            fail!("Couldn't find any non-prefix to parse");
        },
    }
}
