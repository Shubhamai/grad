use crate::{
    // environment::Environment,
    expr::{Expr, ValueType},
    lexer::TokenType,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    // TODO: shell etiquette - use exit code 70 for error
    pub fn visit_expr(&self, expr: &Expr) -> ValueType {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left);
                let right = self.visit_expr(right);

                match operator {
                    TokenType::PLUS => left + right,
                    TokenType::MINUS => left - right,
                    TokenType::STAR => left * right,
                    TokenType::SLASH => left / right,
                    // TokenType::HAT => left.powf(right),
                    TokenType::GREATER => ValueType::Boolean(left > right),
                    TokenType::GREATER_EQUAL => ValueType::Boolean(left >= right),
                    TokenType::LESS => ValueType::Boolean(left < right),
                    TokenType::LESS_EQUAL => ValueType::Boolean(left <= right),
                    TokenType::BANG_EQUAL => ValueType::Boolean(left != right),
                    TokenType::EQUAL_EQUAL => ValueType::Boolean(left == right),

                    _ => panic!("Invalid operator"),
                }
            }
            Expr::Grouping { expression } => self.visit_expr(expression),
            Expr::Literal { value } => value.clone(),
            Expr::Unary { operator, right } => {
                let right = self.visit_expr(right);

                match operator {
                    TokenType::MINUS => -right,
                    TokenType::BANG => !right,
                    _ => panic!("Invalid operator"),
                }
            }
            Expr::Let { token } => {
                todo!("Implement let statement")
            }
        }
    }
}

// TODO : Make it work for - 1 == (2 == 2) - implement int
