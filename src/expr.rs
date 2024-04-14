use crate::lexer::TokenType;

#[derive(Debug, Clone)]
pub enum ValueType {
    Number(f64), // TODO: Ideally, it should be seperate types for int and float (maybe?)
    String(String),
    Boolean(bool),
    Nil,
}


// a struct to represent a AST node
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: TokenType,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: ValueType,
    },
    Unary {
        operator: TokenType,
        right: Box<Expr>,
    },
}


// implement display for Expr
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => {
                write!(f, "({:?} {} {})", operator, left, right)
            }
            Expr::Grouping { expression } => {
                write!(f, "(group {})", expression)
            }
            Expr::Literal { value } => {
                write!(f, "{:?}", value)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({:?} {})", operator, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_pprint() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: TokenType::MINUS,
                right: Box::new(Expr::Literal {
                    value: ValueType::Number(123.0),
                }),
            }),
            operator: TokenType::STAR,
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: ValueType::Number(45.67),
                }),
            }),
        };

        assert_eq!(
            format!("{}", expr),
            "(STAR (MINUS Number(123.0)) (group Number(45.67)))"
        );
    }

}
