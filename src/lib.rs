#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use front::{Expression, parse_grammar};
use middle::convert;
use front::{Terminal, TerminalString, PosLookahead, NegLookahead};

use rustc::plugin::Registry;
use std::gc::Gc;

mod front;
mod libsyn;
mod middle;

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

    match convert(grammar) {
        None => fail!("Conversion didn't work."),
        Some(g) => { // TODO: have to actually generate things
            let parse_fn_str = "parse_".to_string() + g.start.as_str();
            let parse_fn_name = libsyn::Ident::new(libsyn::intern(parse_fn_str.as_slice()));

            let parser_code = generate_parser(cx, g.rules.find(&g.start).unwrap(), parse_fn_name);
            let qi =
                quote_item!(cx,
                    fn $parse_fn_name<'a>(input: &'a str) -> Result<&'a str, String> {
                        $parser_code
                    }
                );
            libsyn::MacItem::new( qi.unwrap() )
        },
    }
}

fn generate_parser(
    cx: &mut libsyn::ExtCtxt,
    expr: &Expression,
    parse_fn_name: libsyn::Ident
) -> Gc<libsyn::Expr> {
    match *expr {
        Terminal(c) => {
            quote_expr!(cx,
                if input.len() > 0 {
                    let cr = input.char_range_at(0);
                    if cr.ch == $c {
                        Ok(input.slice_from(cr.next))
                    } else {
                        Err(format!("Could not match '{}': (saw '{}' instead)",
                                    $c, cr.ch))
                    }
                } else {
                    Err(format!("Could not match '{}' (end of input)", $c))
                }
            )
        },
        TerminalString(ref s) => {
            let sl = s.as_slice();
            let n = s.len();
            let nbytes = s.as_bytes().len();
            quote_expr!(cx,
                if input.len() >= $n {
                    if input.starts_with($sl) {
                        Ok(input.slice_from($nbytes))
                    } else {
                        Err(format!("Could not match '{}': (saw '{}' instead)",
                                    $sl, input))
                    }
                } else {
                    Err(format!("Could not match '{}' (end of input)", $sl))
                }
            )
        },
        PosLookahead(ref e) => {
            fail!("Unimplemented")
        },
        NegLookahead(ref e) => {
            fail!("Unimplemented")
        },
        _ => fail!("Unimplemented"),
    }
}
