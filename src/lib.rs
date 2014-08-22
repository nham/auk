#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use front::parse_grammar;
use middle::convert;
use front::{Terminal, TerminalString};

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

          let parse_func_str = "parse_".to_string() + g.start.as_str();
          let parse_func = libsyn::Ident::new(libsyn::intern(parse_func_str.as_slice()));

          let qi = match *g.rules.find(&g.start).unwrap() {
              Terminal(c) => {
                  quote_item!(cx,
                      fn $parse_func<'a>(input: &'a str) -> Result<&'a str, String> {
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
                      }
                  )
              },
              TerminalString(ref s) => {
                  let sl = s.as_slice();
                  let n = s.len();
                  let nbytes = s.as_bytes().len();
                  quote_item!(cx,
                      fn $parse_func<'a>(input: &'a str) -> Result<&'a str, String> {
                          if input.len() >= $n {
                              if input.starts_with($sl) {
                                  Ok(input.slice_from($nbytes))
                              } else {
                                  Err(format!("Could not match '{}': (saw '{}' instead)", $sl, input))
                              }
                          } else {
                              Err(format!("Could not match '{}' (end of input)", $sl))
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
