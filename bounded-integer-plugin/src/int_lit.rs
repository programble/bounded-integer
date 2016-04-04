use std::cmp::Ordering;

use syntax::ast::{
    Expr,
    ExprKind,
    Ident,
    LitIntType,
    LitKind,
    UnOp,
};
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;

/// Signed integer literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLit {
    /// Negative.
    pub neg: bool,

    /// Integer.
    pub int: u64,
}

impl IntLit {
    /// Extracts an integer literal from an expression.
    pub fn from_expr(expr: &Expr) -> Result<Self, ()> {
        match expr.node {
            ExprKind::Lit(ref lit) => match lit.node {
                LitKind::Int(i, _) => Ok(IntLit { neg: false, int: i }),
                _ => Err(()),
            },
            ExprKind::Unary(UnOp::Neg, ref expr) => match IntLit::from_expr(&*expr) {
                Ok(IntLit { int: 0, .. }) => Err(()),
                Ok(l) => Ok(IntLit { neg: true, ..l }),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    /// Creates an integer literal expression.
    pub fn into_expr(self, cx: &ExtCtxt, sp: Span) -> P<Expr> {
        let lit = cx.expr_lit(sp, LitKind::Int(self.int, LitIntType::Unsuffixed));
        if self.neg {
            cx.expr_unary(sp, UnOp::Neg, lit)
        } else {
            lit
        }
    }

    /// Creates an ident for an integer literal of the form `...N1, Z0, P1...`.
    pub fn into_ident(self, cx: &ExtCtxt) -> Ident {
        let prefix = match (self.neg, self.int) {
            (true, _) => 'N',
            (false, 0) => 'Z',
            (false, _) => 'P',
        };
        cx.ident_of(&format!("{}{}", prefix, self.int))
    }

    /// Returns the successive integer literal.
    pub fn succ(self) -> Self {
        match (self.neg, self.int) {
            (true, 1) => IntLit { neg: false, int: 0 },
            (true, i) => IntLit { neg: true, int: i - 1 },
            (false, i) => IntLit { neg: false, int: i + 1 },
        }
    }
}

impl PartialOrd for IntLit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.neg, other.neg) {
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (true, true) => other.int.partial_cmp(&self.int),
            (false, false) => self.int.partial_cmp(&other.int),
        }
    }
}

impl Ord for IntLit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
