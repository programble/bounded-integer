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
use syntax::ast::{TokenTree, Ident};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use syntax::errors::DiagnosticBuilder;
use syntax::parse;
use syntax::parse::token::keywords::Keyword;

#[derive(Debug)]
struct IntEnum {
    is_pub: bool,
    ident: Ident,
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bounded_integer", expand_bounded_integer);
}

fn expand_bounded_integer(
    cx: &mut ExtCtxt,
    sp: Span,
    tts: &[TokenTree]
) -> Box<MacResult + 'static> {
    let int_enum = match parse_tts(cx, tts) {
        Ok(ie) => ie,
        Err(mut err) => {
            err.emit();
            return DummyResult::any(sp);
        }
    };

    println!("{:?}", int_enum);

    DummyResult::any(sp)
}

fn parse_tts<'a>(cx: &'a mut ExtCtxt, tts: &[TokenTree]) -> Result<IntEnum, DiagnosticBuilder<'a>> {
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_vec());

    let is_pub = parser.eat_keyword(Keyword::Pub);
    try!(parser.expect_keyword(Keyword::Enum));
    let ident = try!(parser.parse_ident());

    Ok(IntEnum {
        is_pub: is_pub,
        ident: ident,
    })
}
