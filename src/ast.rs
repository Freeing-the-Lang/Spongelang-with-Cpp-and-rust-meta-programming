#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    }
}
