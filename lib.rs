#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use expr::PEGExpr;
use util::ident_to_str;

use rustc::plugin::Registry;

mod libsyn;
mod expr;
mod util;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  reg.register_macro("auk", expand)
}

fn expand(
    cx: &mut libsyn::ExtCtxt, 
    _sp: libsyn::Span, 
    tts: &[libsyn::TokenTree]
) -> Box<libsyn::MacResult> {

  let mut parser = libsyn::new_parser_from_tts(cx.parse_sess(),
                                               cx.cfg(),
                                               Vec::from_slice(tts));

  let grammar = parse_grammar(&mut parser);

}

fn parse_grammar(parser: &mut libsyn::Parser) -> Grammar {
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


pub struct Grammar {
    name: libsyn::Ident,
    rules: Vec<Rule>,
}

pub struct Rule {
    name: libsyn::Ident,
    def: Box<Expression>
}

type Expression = PEGExpr<char, libsyn::Ident>;
