use crate::{
    expr::{Expr, Statement, ValueType},
    lexer::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        // Ok(self.expression())

        let mut statements = Vec::new();
        while !self.is_at_end() {
            // statements.push(self.statement());
            statements.push(self.declaration());
        }

        return Ok(statements);
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn declaration(&mut self) -> Statement {
        if self.match_token(&[TokenType::LET]) {
            return self.let_declaration();
        }

        self.statement()

        // TODO : No synchronize()
    }

    fn statement(&mut self) -> Statement {
        // parse print statement in  the form of print LEFT_PAREN expression RIGHT_PAREN SEMICOLON

        if self.match_token(&[TokenType::PRINT]) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Statement {
        // let value = self.expression();
        // let _ = self.consume(TokenType::SEMICOLON, "Expect ';' after value.");

        let _ = self.consume(TokenType::LEFT_PAREN, "Expect '(' after print.");
        let value = self.expression();
        let _ = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
        let _ = self.consume(TokenType::SEMICOLON, "Expect ';' after value.");

        Statement::Print { expression: value }
    }

    fn let_declaration(&mut self) -> Statement {
        let name = self.consume(TokenType::Identifier, "Expect variable name.");

        let _ = self.consume(TokenType::EQUAL, "Expect '=' after variable name.");

        let initializer = self.expression();

        let _ = self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        );

        Statement::Let {
            name: match name {
                Ok(name) => name.lexeme,
                _ => panic!("Expect variable name."),
            },
            initializer,
        }
    }

    fn expression_statement(&mut self) -> Statement {
        let expr = self.expression();
        let _ = self.consume(TokenType::SEMICOLON, "Expect ';' after expression.");

        Statement::Expression { expression: expr }
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();

            // create a new Expr::Binary
            // with the left being the previous expression
            // the operator being the current token
            // and the right being the next expression
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS, TokenType::HAT]) {
            let operator = self.previous();
            let right = self.factor();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();

            return Expr::Unary {
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&[TokenType::True]) {
            return Expr::Literal {
                value: ValueType::Boolean(true),
            };
        }

        if self.match_token(&[TokenType::False]) {
            return Expr::Literal {
                value: ValueType::Boolean(false),
            };
        }

        if self.match_token(&[TokenType::NIL]) {
            return Expr::Literal {
                value: ValueType::Nil,
            };
        }

        if self.match_token(&[TokenType::Number]) {
            return Expr::Literal {
                value: ValueType::Number(match self.previous().literal {
                    Some(ValueType::Number(n)) => n,
                    _ => panic!("Expect number literal."),
                }),
            };
        }

        if self.match_token(&[TokenType::String]) {
            return Expr::Literal {
                value: ValueType::String(match self.previous().literal {
                    Some(ValueType::String(s)) => s,
                    _ => panic!("Expect string literal."),
                }),
            };
        }

        if self.match_token(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            let _ = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        if self.match_token(&[TokenType::LET]) {
            println!("Let statement");
            println!("{:?}", self.previous());
            return Expr::Let {
                token: self.previous(),
            };
        }

        // COMMENT token
        if self.match_token(&[TokenType::COMMENT]) {
            return self.primary();
        }

        panic!("Expect expression.");
        // panic!("Error at {}: Expect expression.", self.peek().span.start);
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if !self.check(&token_type) {
            // Err(format!("Error at {}: {}", self.peek().span.start, message))
            panic!("{}", message);
        } else {
            self.advance();
            let token = self.previous();
            Ok(token)
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|t| self.check(t)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        // is at end if the current token is last token
        &self.tokens.len() == &self.current
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token {
                token_type: TokenType::LEFT_PAREN,
                lexeme: String::from("("),
                literal: None,
                span: 0..1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("1"),
                literal: Some(ValueType::Number(1.0)),
                span: 1..2,
            },
            Token {
                token_type: TokenType::PLUS,
                lexeme: String::from("+"),
                literal: None,
                span: 2..3,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("2"),
                literal: Some(ValueType::Number(2.0)),
                span: 3..4,
            },
            Token {
                token_type: TokenType::RIGHT_PAREN,
                lexeme: String::from(")"),
                literal: None,
                span: 4..5,
            },
            Token {
                token_type: TokenType::MINUS,
                lexeme: String::from("-"),
                literal: None,
                span: 5..6,
            },
            Token {
                token_type: TokenType::LEFT_PAREN,
                lexeme: String::from("("),
                literal: None,
                span: 6..7,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("3"),
                literal: Some(ValueType::Number(3.0)),
                span: 7..8,
            },
            Token {
                token_type: TokenType::SLASH,
                lexeme: String::from("/"),
                literal: None,
                span: 8..9,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("2"),
                literal: Some(ValueType::Number(2.0)),
                span: 9..10,
            },
            Token {
                token_type: TokenType::RIGHT_PAREN,
                lexeme: String::from(")"),
                literal: None,
                span: 10..11,
            },
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.expression();

        assert_eq!(format!("{}", expr), "(MINUS (group (PLUS Number(1.0) Number(2.0))) (group (SLASH Number(3.0) Number(2.0))))");
    }
}
