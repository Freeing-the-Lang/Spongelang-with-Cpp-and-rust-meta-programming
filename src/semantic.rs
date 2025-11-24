use crate::ast::Expr;

pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn analyze(expr: &Expr) {
        // 아주 간단한 의미 분석: 숫자 연산만 허용
        fn check(e: &Expr) {
            match e {
                Expr::Number(_) => {}
                Expr::Binary { left, right, .. } => {
                    check(left);
                    check(right);
                }
            }
        }
        check(expr);
    }
}
