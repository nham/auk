use expr::Expression;
use libsyn;

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
    //thing goes here
    parser.expect(&libsyn::RBRACE);
    Grammar { name: name, rules: vec!() }
}

fn consume_grammar_keyword(parser: &mut libsyn::Parser) -> bool {
    // the second value attached to IDENT is the "is_mod_name" flag
    match parser.token {
        libsyn::IDENT(ident, false) if "grammar" == libsyn::get_ident(ident).get() => {
            parser.bump();
            true
        },
        _ => false,
    }
}
