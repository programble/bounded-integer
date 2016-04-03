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
use syntax::ast::{
    Attribute,
    EnumDef,
    Expr,
    ExprKind,
    Ident,
    Item,
    ItemKind,
    LitIntType,
    LitKind,
    TokenTree,
    UnOp,
    Variant,
    Visibility,
};
use syntax::codemap::Span;
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::errors::DiagnosticBuilder;
use syntax::parse::token::{DelimToken, InternedString, Token};
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
        let repr_list = cx.meta_list(sp, InternedString::new("repr"), vec![repr]);
        self.attrs.push(cx.attribute(sp, repr_list));
    }

    /// Generates variants for the range.
    fn variants(&self, cx: &mut ExtCtxt) -> Vec<Variant> {
        let mut vec = Vec::new();
        let mut current = self.min.clone();
        loop {
            let (neg, int) = expr_intlit(&*current).unwrap(); // FIXME
            let mut variant = cx.variant(current.span, intlit_ident(cx, neg, int), vec![]);
            variant.node.disr_expr = Some(current);
            vec.push(variant);

            // FIXME: Infinite loop risk.
            if Ok((neg, int)) == expr_intlit(&*self.max) { break; }
            let (neg, int) = intlit_succ(neg, int);
            current = intlit_expr(cx, self.min.span, neg, int);
        }
        vec
    }

    /// Creates an item from the parsed bounded integer enum.
    fn into_item(self, cx: &mut ExtCtxt, sp: Span) -> P<Item> {
        let is_pub = self.is_pub;
        let enum_def = EnumDef { variants: self.variants(cx) };
        let item_kind = ItemKind::Enum(enum_def, Default::default());
        cx.item(sp, self.name, self.attrs, item_kind).map(|mut item| {
            if is_pub { item.vis = Visibility::Public; }
            item
        })
    }
}

/// Extracts `(neg, int)` from an integer literal expression.
fn expr_intlit(expr: &Expr) -> Result<(bool, u64), ()> {
    match expr.node {
        ExprKind::Lit(ref lit) => match lit.node {
            LitKind::Int(i, _) => Ok((false, i)),
            _ => Err(()),
        },
        ExprKind::Unary(UnOp::Neg, ref expr) => {
            expr_intlit(&*expr).map(|(_, i)| (true, i))
        },
        _ => Err(()),
    }
}

/// Creates an integer literal expression.
fn intlit_expr(cx: &mut ExtCtxt, sp: Span, neg: bool, int: u64) -> P<Expr> {
    let lit = cx.expr_lit(sp, LitKind::Int(int, LitIntType::Unsuffixed));
    if neg {
        cx.expr_unary(sp, UnOp::Neg, lit)
    } else {
        lit
    }
}

/// Returns the successive integer literal.
fn intlit_succ(neg: bool, int: u64) -> (bool, u64) {
    match (neg, int) {
        (true, 1) => (false, 0),
        (true, i) => (true, i - 1),
        (false, i) => (false, i + 1),
    }
}

/// Creates an ident for an integer literal.
fn intlit_ident(cx: &ExtCtxt, neg: bool, int: u64) -> Ident {
    let prefix = match (neg, int) {
        (true, _) => 'N',
        (false, 0) => 'Z',
        (false, _) => 'P',
    };
    cx.ident_of(&format!("{}{}", prefix, int))
}
