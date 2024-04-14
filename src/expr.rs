use crate::lexer::TokenType;

#[derive(Debug, Clone)]
pub enum ValueType {
    Number(f64), // TODO: Ideally, it should be seperate types for int and float (maybe?)
    String(String),
    Boolean(bool),
    Nil,
    // Lists, Dicts, Tensors, etc.
}

impl std::ops::Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a + b),
            (ValueType::String(a), ValueType::String(b)) => ValueType::String(a + &b),
            _ => panic!("Invalid operands for addition"),
        }
    }
}

impl std::ops::Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a - b),
            _ => panic!("Invalid operands for subtraction"),
        }
    }
}

impl std::ops::Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a * b),
            _ => panic!("Invalid operands for multiplication"),
        }
    }
}

impl std::ops::Div for ValueType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a / b),
            _ => panic!("Invalid operands for division"),
        }
    }
}

impl std::ops::Not for ValueType {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            ValueType::Boolean(b) => ValueType::Boolean(!b),
            _ => panic!("Invalid operand for not"),
        }
    }
}

impl std::ops::Neg for ValueType {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            ValueType::Number(n) => ValueType::Number(-n),
            _ => panic!("Invalid operand for negation"),
        }
    }
}

// a struct to represent the AST
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
            Expr::Binary {
                left,
                operator,
                right,
            } => {
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
