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

// This is how we do it
//
// Definition = Identifier SP '=' SP Expression
// Expression = Sequence (SP '/' SP Sequence)*
// Sequence   = Chunk*
// Chunk      = PRED? SP Primary SP AMOUNT?
// Primary    = Identifier !(SP '=')
//             / '(' SP Expression SP ')'
//             / Literal
//             / Class
//             / '.'
//
// Currently we do not parse choices, just a sequence of chunks
// TODO: need to amend this to support parsing of multiple rules
fn parse_rule_expr(parser: &mut libsyn::Parser) -> Expression {
    let mut choices = vec!();
    loop {
        match parser.token {
            libsyn::RBRACE => break,
            libsyn::EOF => break,
            libsyn::RPAREN => break,
            _ => choices.push(parse_rule_choice(parser)),
        }
    }

    if choices.len() == 1 {
        choices.move_iter().next().unwrap()
    } else {
        Alt(choices)
    }
}

// parse a sequence of chunks. this forms one "choice", e.g. if we have the
// the expression:
//
//     choice1 / choice2 / ...
//
// then this function will parse choice1
fn parse_rule_choice(parser: &mut libsyn::Parser) -> Expression {
    let mut chunks = vec!();
    loop {
        match parser.token {
            libsyn::RBRACE => break,
            libsyn::EOF => break,
            libsyn::RPAREN => break,
            libsyn::BINOP(libsyn::SLASH) => {
                parser.bump();
                break;
            },
            _ => chunks.push(parse_rule_chunk(parser)),
        }
    }

    if chunks.len() == 1 {
        chunks.move_iter().next().unwrap()
    } else {
        Seq(chunks)
    }
}

fn parse_rule_chunk(parser: &mut libsyn::Parser) -> Expression {
    match parser.token {
        libsyn::BINOP(libsyn::AND) => {
            parser.bump();
            return PosLookahead(box parse_rule_chunk(parser));
        },
        libsyn::NOT => {
            parser.bump();
            return NegLookahead(box parse_rule_chunk(parser));
        },
        _ => parse_rule_chunk_no_prefix(parser),
    }
}


fn parse_rule_chunk_no_prefix(parser: &mut libsyn::Parser) -> Expression {
    let expr = parse_primary(parser);
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
}


// A 'primary' is a char, a string, a dot, a character class, a parenthesized
// expression or a non-terminal
// TODO: implement non-terminals, parens
fn parse_primary(parser: &mut libsyn::Parser) -> Expression {
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

        },
        libsyn::LPAREN => {
            parser.bump();
            let expr = parse_rule_expr(parser);
            match parser.token {
                libsyn::RPAREN => {
                    parser.bump();
                    expr
                },
                _ => fail!("Mismatched parens"),
            }
        },
        _ => {
            fail!("Couldn't find any non-prefix to parse");
        },
    }
}

#[cfg(test)]
mod test {
    use syntax::parse::{ParseSess, new_parser_from_source_str, new_parse_sess};
    use syntax::parse::parser::Parser;

    use super::parse_rule_expr;
    use super::{Terminal, AnyTerminal, TerminalString, Class, Optional,
                ZeroOrMore, OneOrMore, PosLookahead, NegLookahead, Seq, Alt};

    macro_rules! is_variant0(
        ($e:expr, $i:ident) => (match $e { $i => true, _ => false })
    )

    macro_rules! is_variant1(
        ($e:expr, $i:ident) => (match $e { $i(_) => true, _ => false })
    )

    fn new_parser<'a>(s: &str, sess: &'a ParseSess) -> Parser<'a> {
        new_parser_from_source_str(sess, vec!(),
                                   "bogus".to_string(),
                                   s.to_string())
    }

    #[test]
    fn test_parse_char() {
        let sess = new_parse_sess();
        let mut p = new_parser("'c'", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), Terminal) );
    }

    #[test]
    fn test_parse_dot() {
        let sess = new_parse_sess();
        let mut p = new_parser(".", &sess);
        assert!( is_variant0!(parse_rule_expr(&mut p), AnyTerminal) );
    }

    #[test]
    fn test_parse_str() {
        let sess = new_parse_sess();
        let mut p = new_parser("\"abc\"", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), TerminalString) );
    }

    #[test]
    fn test_parse_class() {
        let sess = new_parse_sess();
        let mut p = new_parser("[\"abc\"]", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), Class) );
    }

    #[test]
    fn test_parse_optional() {
        let sess = new_parse_sess();
        let mut p = new_parser("[\"abc\"]?", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), Optional) );
    }

    #[test]
    fn test_parse_zeroormore() {
        let sess = new_parse_sess();
        let mut p = new_parser("[\"abc\"]*", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), ZeroOrMore) );
    }

    #[test]
    fn test_parse_oneormore() {
        let sess = new_parse_sess();
        let mut p = new_parser("[\"abc\"]+", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), OneOrMore) );
    }

    #[test]
    fn test_parse_poslookahead() {
        let sess = new_parse_sess();
        let mut p = new_parser("&[\"abc\"]+", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), PosLookahead) );
    }

    #[test]
    fn test_parse_neglookahead() {
        let sess = new_parse_sess();
        let mut p = new_parser("![\"abc\"]+", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), NegLookahead) );
    }

    #[test]
    fn test_parse_seq() {
        let sess = new_parse_sess();
        let mut p = new_parser("![\"abc\"]+ &.", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), Seq) );
    }

    #[test]
    fn test_parse_alt() {
        let sess = new_parse_sess();
        let mut p = new_parser("![\"abc\"]+ / 'e'*", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), Alt) );
    }

    #[test]
    fn test_parse_parens() {
        let sess = new_parse_sess();
        let mut p = new_parser("!([\"abc\"]+ / ('e' \"abc\")*)", &sess);
        assert!( is_variant1!(parse_rule_expr(&mut p), NegLookahead) );
    }
}
