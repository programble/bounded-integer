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

/// A signed integer literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLit {
    neg: bool,
    int: u64,
}

impl IntLit {
    /// Extracts an integer literal from an expression.
    pub fn from_expr(expr: &Expr) -> Result<Self, ()> {
        match expr.node {
            ExprKind::Lit(ref lit) => match lit.node {
                LitKind::Int(i, _) => Ok(IntLit { neg: false, int: i }),
                _ => Err(()),
            },
            ExprKind::Unary(UnOp::Neg, ref expr) => {
                IntLit::from_expr(&*expr).map(|l| IntLit { neg: true, ..l })
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

    /// Creates an ident for an integer literal.
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
