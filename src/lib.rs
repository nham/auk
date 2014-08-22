#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use front::parse_grammar;
use middle::convert;
use front::{Terminal};

use rustc::plugin::Registry;

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
          let qi = match *g.rules.find(&g.start).unwrap() {
              Terminal(c) => {
                  quote_item!(cx,
    fn parse_char<'a>(input: &'a str) -> Result<&'a str, String> {
        if input.len() > 0 {
            let cr = input.char_range_at(0);
            if cr.ch == $c {
                Ok(input.slice_from(cr.next))
            } else {
                Err(format!("Could not match '{}': (saw '{}' instead)", $c, cr.ch))
            }
        } else {
            Err(format!("Could not match '{}' (end of input)", $c))
        }
    }
                )

              },

              _ => fail!("Unimplemented"),
          };

          libsyn::MacItem::new( qi.unwrap() )
      },
  }

}
