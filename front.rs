
use expr::PEGExpr;
use libsyn;
use util::ident_to_str;

pub struct Grammar {
    name: libsyn::Ident,
    rules: Vec<Rule>,
}

pub struct Rule {
    name: libsyn::Ident,
    def: Box<Expression>
}

type Expression = PEGExpr<char, libsyn::Ident>;


pub fn parse_grammar(parser: &mut libsyn::Parser) -> Grammar {
    if !consume_grammar_keyword(parser) {
        let span = parser.span;
        parser.span_fatal(span,
            format!("Expected grammar declaration of the form `grammar <name> \
                    {{...}}` but found `{}`", parser.this_token_to_string()
                   ).as_slice());
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
        libsyn::IDENT(ident, false) if "grammar" == ident_to_str(ident) => {
            parser.bump();
            true
        },
        _ => false,
    }
}
