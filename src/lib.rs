#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use front::parse_grammar;
use middle::convert;
pub use peg::PEGGrammar;
pub use expr::Empty;

use rustc::plugin::Registry;

mod expr;
mod front;
mod libsyn;
mod middle;
mod peg;

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
      Some(_) => { // TODO: have to actually generate things
          let qi = quote_expr!(cx, auk::PEGGrammar::new());
          libsyn::MacExpr::new( qi )
      },
  }

}
