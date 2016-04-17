//! Provides the procedural macro `bounded_integer!` for generating enums for the
//! [`bounded-integer`][main] crate.
//!
//! [main]: https://cmcenroe.me/bounded-integer/bounded_integer
//!
//! # Syntax
//!
//! The syntax parsed by `bounded_integer!` is roughly equivalent to the following
//! `macro_rules!`-like matcher.
//!
//! ```ignore
//! $(#[$attr:meta])*
//! $(pub)? enum $name:ident: $repr:ident { $min:expr...$max:expr }
//! ```
//!
//! Which expands to the following:
//!
//! ```ignore
//! $(#[$attr:meta])*
//! #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
//! #[repr($repr)]
//! $(pub)? enum $name { ... }
//! bounded_integer_impls!($name, $repr, ..., ...);
//! ```
//!
//! Where `...` are variants of the form `...N1, Z0, P1...`.
//!
//! Note that the `bounded_integer_impls!` macro is provided by the [`bounded-integer`][main]
//! crate.

#![feature(plugin_registrar, rustc_private)]

#![warn(
    missing_docs,
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
pub fn expand_bounded_integer(
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
    let items = integer_enum.into_items(cx, sp);
    MacEager::items(SmallVector::many(items))
}
