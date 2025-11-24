use crate::ast::Expr;

pub struct AsmCodegen;

impl AsmCodegen {

    pub fn emit(expr: &Expr) -> String {
        let value = Self::eval(expr);

        format!(
r#"section .text
global _start

_start:
    mov rax, 60
    mov rdi, {value}
    syscall
"#
        )
    }

    fn eval(expr: &Expr) -> i64 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Binary { left, op, right } => {
                let l = Self::eval(left);
                let r = Self::eval(right);

                match op.as_str() {
                    "+" => l + r,
                    "-" => l - r,
                    _ => panic!("Unknown operator")
                }
            }
        }
    }
}
