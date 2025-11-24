use crate::ast::Expr;

pub struct RustMacroBridge;

impl RustMacroBridge {
    pub fn expand(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => format!("sponge_num!({})", n),
            Expr::Binary { left, op, right } => {
                format!(
                    "sponge_binop!({}, \"{}\", {})",
                    Self::expand(left),
                    op,
                    Self::expand(right)
                )
            }
        }
    }
}
