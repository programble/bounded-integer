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

pub use integer_enum::IntegerEnum;
mod integer_enum;

pub use int_lit::IntLit;
mod int_lit;

use rustc_plugin::Registry;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::util::small_vector::SmallVector;

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
    let integer_enum = match IntegerEnum::parse_tts(cx, tts) {
        Ok(e) => e,
        Err(mut err) => {
            err.emit();
            return DummyResult::any(sp);
        },
    };
    let item = integer_enum.into_item(cx, sp);
    MacEager::items(SmallVector::one(item))
}
