#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use front::{Expression, parse_grammar};
use middle::convert;
use front::{Terminal, AnyTerminal, TerminalString, PosLookahead, NegLookahead,
            Class, ZeroOrMore, OneOrMore, Optional, Seq, Alt};

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
    let parse_fn_name = grammar.name;

    match convert(grammar) {
        None => fail!("Conversion didn't work."),
        Some(g) => {
            let input = libsyn::Ident::new(libsyn::intern("input"));

            let parser_code = generate_parser(cx,
                                              g.rules.find(&g.start).unwrap(),
                                              parse_fn_name,
                                              input);
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
    fn_name: libsyn::Ident,
    input_ident: libsyn::Ident,
) -> Gc<libsyn::Expr> {
    match *expr {
        Terminal(c) => {
            quote_expr!(cx,
                if $input_ident.len() > 0 {
                    let cr = $input_ident.char_range_at(0);
                    if cr.ch == $c {
                        Ok($input_ident.slice_from(cr.next))
                    } else {
                        Err(format!("Could not match '{}': (saw '{}' instead)",
                                    $c, cr.ch))
                    }
                } else {
                    Err(format!("Could not match '{}' (end of input)", $c))
                }
            )
        },
        AnyTerminal => {
            quote_expr!(cx,
                if $input_ident.len() > 0 {
                    let cr = $input_ident.char_range_at(0);
                    Ok($input_ident.slice_from(cr.next))
                } else {
                    Err(format!("Could not match '.' (end of input)"))
                }
            )
        },
        TerminalString(ref s) => {
            let sl = s.as_slice();
            let n = s.len();
            let nbytes = s.as_bytes().len();
            quote_expr!(cx,
                if $input_ident.len() >= $n {
                    if $input_ident.starts_with($sl) {
                        Ok($input_ident.slice_from($nbytes))
                    } else {
                        Err(format!("Could not match '{}': (saw '{}' instead)",
                                    $sl, $input_ident))
                    }
                } else {
                    Err(format!("Could not match '{}' (end of input)", $sl))
                }
            )
        },
        PosLookahead(ref e) => {
            let parser = generate_parser(cx, &**e, fn_name, input_ident);
            quote_expr!(cx,
                match $parser {
                    Ok(_) => Ok($input_ident),
                    Err(e) => Err(e),
                }
            )
        },
        NegLookahead(ref e) => {
            let parser = generate_parser(cx, &**e, fn_name, input_ident);
            quote_expr!(cx,
                match $parser {
                    Ok(_) => Err(format!("Could not match ! expression")),
                    Err(e) => Ok($input_ident),
                }
            )
        },
        Class(ref s) => {
            let sl = s.as_slice();
            quote_expr!(cx,
                if $input_ident.len() > 0 {
                    let cr = $input_ident.char_range_at(0);
                    if $sl.find(cr.ch).is_some() {
                        Ok($input_ident.slice_from(cr.next))
                    } else {
                        Err(format!("Could not match '[{}]': (saw '{}' instead)",
                                    $sl, cr.ch))
                    }
                } else {
                    Err(format!("Could not match '[{}]' (end of input)", $sl))
                }
            )
        },
        ZeroOrMore(ref e) => {
            let parser = generate_parser(cx, &**e, fn_name, input_ident);
            quote_expr!(cx,
                match $parser {
                    Ok(rem) => $fn_name(rem),
                    Err(e) => Ok($input_ident),
                }
            )
        },
        OneOrMore(ref e) => {
            let parser = generate_parser(cx, &**e, fn_name, input_ident);
            quote_expr!(cx,
                match $parser {
                    Ok(rem) => {
                        match $fn_name(rem) {
                            Ok(r) => Ok(r),
                            Err(_) => Ok(rem),
                        }
                    },
                    Err(e) => Err(e),
                }
            )
        },
        Optional(ref e) => {
            let parser = generate_parser(cx, &**e, fn_name, input_ident);
            quote_expr!(cx,
                match $parser {
                    Ok(rem) => Ok(rem),
                    Err(e) => Ok($input_ident),
                }
            )
        },
        Seq(ref v) => {
            if v.len() == 0 {
                fail!("Can't interpret a sequence of zero length");
            } else {
                generate_seq_parser(cx, v.as_slice(), fn_name, input_ident)
            }
        },
        Alt(ref v) => {
            if v.len() == 0 {
                fail!("Can't interpret a sequence of zero length");
            } else {
                generate_alt_parser(cx, v.as_slice(), fn_name, input_ident)
            }
        },
        _ => fail!("Unimplemented"),
    }
}

fn generate_seq_parser(
    cx: &mut libsyn::ExtCtxt,
    exprs: &[Expression],
    fn_name: libsyn::Ident,
    input_ident: libsyn::Ident,
) -> Gc<libsyn::Expr> {
    if exprs.len() == 0 {
        fail!("Don't call generate_seq_parser with a slice of length 0")
    } else if exprs.len() == 1 {
        let parser = generate_parser(cx, &exprs[0], fn_name, input_ident);
        quote_expr!(cx, $parser)
    } else {
        let parser = generate_parser(cx, &exprs[0], fn_name, input_ident);
        let rem = libsyn::Ident::new(libsyn::intern("rem"));
        let parser2 = generate_seq_parser(cx, exprs.slice_from(1), fn_name, rem);
        quote_expr!(cx,
            match $parser {
                Err(e) => Err(e),
                Ok(rem) => $parser2,
            }
        )
    }
}

fn generate_alt_parser(
    cx: &mut libsyn::ExtCtxt,
    exprs: &[Expression],
    fn_name: libsyn::Ident,
    input_ident: libsyn::Ident,
) -> Gc<libsyn::Expr> {
    if exprs.len() == 0 {
        fail!("Don't call generate_alt_parser with a slice of length 0")
    } else if exprs.len() == 1 {
        let parser = generate_parser(cx, &exprs[0], fn_name, input_ident);
        quote_expr!(cx, $parser)
    } else {
        let parser = generate_parser(cx, &exprs[0], fn_name, input_ident);
        let parser2 = generate_alt_parser(cx, exprs.slice_from(1), fn_name, input_ident);
        quote_expr!(cx,
            match $parser {
                Err(e) => $parser2,
                Ok(rem) => Ok(rem),
            }
        )

    }
}
