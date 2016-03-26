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

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bounded_integer", expand_bounded_integer);
}

fn expand_bounded_integer(
    _context: &mut ExtCtxt,
    span: Span,
    _args: &[TokenTree]
) -> Box<MacResult + 'static> {
    DummyResult::any(span)
}
