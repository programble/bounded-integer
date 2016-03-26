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
use syntax::ast::{TokenTree, Ident, Expr, EnumDef, Visibility};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::errors::DiagnosticBuilder;
use syntax::parse::token::{Token, DelimToken};
use syntax::parse::token::keywords::Keyword;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

/// Parsed bounded integer enum.
#[derive(Debug)]
struct IntegerEnum {
    is_pub: bool,
    name: Ident,
    repr: Ident,
    min: P<Expr>,
    max: P<Expr>,
}

/// Registers the `bounded_integer!` macro.
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bounded_integer", expand_bounded_integer);
}

/// Expands the `bounded_integer!` macro.
fn expand_bounded_integer(
    cx: &mut ExtCtxt,
    sp: Span,
    tts: &[TokenTree],
) -> Box<MacResult + 'static> {
    let integer_enum = match parse_tts(cx, tts) {
        Ok(ie) => ie,
        Err(mut err) => {
            err.emit();
            return DummyResult::any(sp);
        },
    };

    let enum_def = EnumDef { variants: Vec::new() };
    let enum_item = cx.item_enum(sp, integer_enum.name, enum_def).map(|mut item| {
        if integer_enum.is_pub {
            item.vis = Visibility::Public;
        }
        item
    });

    MacEager::items(SmallVector::one(enum_item))
}

/// Parses the argument token trees into an `IntegerEnum`.
///
/// ```text
/// [pub] enum $name: $repr { $min...$max }
/// ```
fn parse_tts<'a>(
    cx: &'a mut ExtCtxt,
    tts: &[TokenTree],
) -> Result<IntegerEnum, DiagnosticBuilder<'a>> {
    let mut parser = cx.new_parser_from_tts(tts);

    // pub enum
    let is_pub = parser.eat_keyword(Keyword::Pub);
    try!(parser.expect_keyword(Keyword::Enum));

    // $name: $repr
    let name = try!(parser.parse_ident());
    try!(parser.expect(&Token::Colon));
    let repr = try!(parser.parse_ident());

    // { $min...$max }
    try!(parser.expect(&Token::OpenDelim(DelimToken::Brace)));
    let min = try!(parser.parse_pat_literal_maybe_minus());
    try!(parser.expect(&Token::DotDotDot));
    let max = try!(parser.parse_pat_literal_maybe_minus());
    try!(parser.expect(&Token::CloseDelim(DelimToken::Brace)));

    Ok(IntegerEnum {
        is_pub: is_pub,
        name: name,
        repr: repr,
        min: min,
        max: max,
    })
}
