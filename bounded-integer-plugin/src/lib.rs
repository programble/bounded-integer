#![feature(plugin_registrar, rustc_private)]

#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
)]

extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use syntax::errors::DiagnosticBuilder;
use syntax::parse;
use syntax::parse::token::keywords::Keyword;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bounded_integer", expand_bounded_integer);
}

fn expand_bounded_integer(
    cx: &mut ExtCtxt,
    sp: Span,
    tts: &[TokenTree]
) -> Box<MacResult + 'static> {
    match parse_tts(cx, tts) {
        Err(mut e) => e.emit(),
        _ => (),
    };
    DummyResult::any(sp)
}

fn parse_tts<'a>(cx: &'a mut ExtCtxt, tts: &[TokenTree]) -> Result<(), DiagnosticBuilder<'a>> {
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_vec());

    let is_pub = parser.eat_keyword(Keyword::Pub);
    try!(parser.expect_keyword(Keyword::Enum));

    Ok(())
}
