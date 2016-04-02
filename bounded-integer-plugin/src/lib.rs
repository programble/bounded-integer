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
use syntax::ast::{TokenTree, Ident, Expr, EnumDef, Visibility, Attribute, ItemKind, Item};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::errors::DiagnosticBuilder;
use syntax::parse::token::{Token, DelimToken, InternedString};
use syntax::parse::token::keywords::Keyword;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

/// Parsed bounded integer enum.
#[derive(Debug)]
struct IntegerEnum {
    attrs: Vec<Attribute>,
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
    let mut integer_enum = match parse_tts(cx, tts) {
        Ok(ie) => ie,
        Err(mut err) => {
            err.emit();
            return DummyResult::any(sp);
        },
    };
    integer_enum.add_derives(cx, sp);
    integer_enum.add_repr(cx, sp);
    let item = integer_enum.into_item(cx, sp);
    MacEager::items(SmallVector::one(item))
}

/// Parses the argument token trees into an `IntegerEnum`.
///
/// ```text
/// $(#[$attr:meta])*
/// [pub] enum $name:ident: $repr:ident { $min:expr...$max:expr }
/// ```
fn parse_tts<'a>(
    cx: &'a mut ExtCtxt,
    tts: &[TokenTree],
) -> Result<IntegerEnum, DiagnosticBuilder<'a>> {
    let mut parser = cx.new_parser_from_tts(tts);

    // $(#[$attr:meta])*
    let attrs = try!(parser.parse_outer_attributes());

    // [pub] enum
    let is_pub = parser.eat_keyword(Keyword::Pub);
    try!(parser.expect_keyword(Keyword::Enum));

    // $name:ident: $repr:ident
    let name = try!(parser.parse_ident());
    try!(parser.expect(&Token::Colon));
    let repr = try!(parser.parse_ident());

    // { $min:expr...$max:expr }
    try!(parser.expect(&Token::OpenDelim(DelimToken::Brace)));
    let min = try!(parser.parse_pat_literal_maybe_minus());
    try!(parser.expect(&Token::DotDotDot));
    let max = try!(parser.parse_pat_literal_maybe_minus());
    try!(parser.expect(&Token::CloseDelim(DelimToken::Brace)));

    Ok(IntegerEnum {
        attrs: attrs,
        is_pub: is_pub,
        name: name,
        repr: repr,
        min: min,
        max: max,
    })
}

impl IntegerEnum {
    /// Adds `#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]` to the attributes.
    fn add_derives(&mut self, cx: &mut ExtCtxt, sp: Span) {
        let derives = ["Clone", "Copy", "PartialEq", "Eq", "PartialOrd", "Ord"].iter()
            .map(|s| InternedString::new(s))
            .map(|s| cx.meta_word(sp, s))
            .collect();
        let derive_list = cx.meta_list(sp, InternedString::new("derive"), derives);
        self.attrs.push(cx.attribute(sp, derive_list));
    }

    /// Adds `#[repr($repr)]` to the attributes.
    fn add_repr(&mut self, cx: &mut ExtCtxt, sp: Span) {
        let repr = cx.meta_word(sp, self.repr.name.as_str());
        let repr_list = cx.meta_list(sp, InternedString::new("repr"), vec![repr_meta]);
        self.attrs.push(cx.attribute(sp, repr_list));
    }

    /// Creates an item from the parsed bounded integer enum.
    fn into_item(self, cx: &mut ExtCtxt, sp: Span) -> P<Item> {
        let is_pub = self.is_pub;

        let enum_def = EnumDef {
            variants: vec![
                cx.variant(sp, cx.ident_of("Dummy"), vec![]),
            ],
        };

        let item_kind = ItemKind::Enum(enum_def, Default::default());
        cx.item(sp, self.name, self.attrs, item_kind).map(|mut item| {
            if is_pub { item.vis = Visibility::Public; }
            item
        })
    }
}
