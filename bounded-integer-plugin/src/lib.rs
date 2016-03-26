#![feature(plugin_registrar, rustc_private)]

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
