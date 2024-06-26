use core::fmt;

use crate::scanner::{Lexer, TokenType};
use serde::{Deserialize, Serialize};

/// Represents a node in the Abstract Syntax Tree (AST)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ASTNode {
    IntNumber(i64),
    FloatNumber(f64),
    Identifier(String),
    Boolean(bool),
    String(String),
    Op(Ops, Vec<ASTNode>),
    Callee(String, Vec<ASTNode>),
    Let(String, Vec<ASTNode>),
    Assign(String, Vec<ASTNode>),
    If(Vec<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>),
    While(Vec<ASTNode>, Vec<ASTNode>),
    Print(Vec<ASTNode>),
    Function(String, Vec<String>, Vec<ASTNode>),
    Block(Vec<ASTNode>),
}

/// Represents binary operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    At,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Represents unary operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate,
    Not,
}

/// Represents postfix operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PostfixOp {
    Index,
    Call,
    StarStar,
}

/// Combines all operation types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Ops {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    PostfixOp(PostfixOp),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(TokenType, String),
    MissingToken(TokenType, String),
    InvalidOperator(String),
    SyntaxError(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token, context) => {
                write!(f, "Unexpected token {:?} {}", token, context)
            }
            ParseError::MissingToken(token, context) => {
                write!(f, "Missing token {:?} {}", token, context)
            }
            ParseError::InvalidOperator(msg) => write!(f, "Invalid operator: {}", msg),
            ParseError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

type ParseResult<T> = Result<T, ParseError>;

/// Parser struct for recursive descent parsing
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        Parser { lexer }
    }

    /// Main parsing function
    pub fn parse(&mut self) -> ParseResult<Vec<ASTNode>> {
        let mut statements = vec![];

        while self.lexer.peek().token_type != TokenType::EOF {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> ParseResult<ASTNode> {
        let statement = match self.lexer.peek().token_type {
            TokenType::PRINT => self.parse_print(),
            TokenType::LET => self.parse_let(),
            TokenType::FN => self.parse_function(),
            TokenType::LeftBrace => self.parse_block(),
            TokenType::IF => self.parse_if(),
            TokenType::WHILE => self.parse_while(),
            TokenType::Identifier if self.is_assignment() => self.parse_assign(),
            TokenType::SEMICOLON => {
                self.lexer.next(); // Consume the semicolon
                return Ok(ASTNode::Block(vec![])); // Return an empty block for lone semicolons
            }
            _ => self.parse_expression(),
        }?;

        // Consume the semicolon if present
        if self.lexer.peek().token_type == TokenType::SEMICOLON {
            self.lexer.next();
        }

        Ok(statement)
    }
    fn parse_print(&mut self) -> ParseResult<ASTNode> {
        self.lexer.next();
        if self.lexer.next().token_type != TokenType::LeftParen {
            return Err(ParseError::SyntaxError(
                "Expected '(' after print".to_string(),
            ));
        }
        let expr = self.parse_expression()?;
        if self.lexer.next().token_type != TokenType::RightParen {
            return Err(ParseError::MissingToken(
                TokenType::RightParen,
                "to close print statement".to_string(),
            ));
        }
        Ok(ASTNode::Print(vec![expr]))
    }

    fn parse_let(&mut self) -> ParseResult<ASTNode> {
        self.lexer.next();
        let identifier = self.lexer.next().lexeme;
        if self.lexer.next().token_type != TokenType::EQUAL {
            return Err(ParseError::MissingToken(
                TokenType::EQUAL,
                "to assign value to variable".to_string(),
            ));
        }
        let expr = self.parse_expression()?;
        Ok(ASTNode::Let(identifier, vec![expr]))
    }

    // TODO: might need fixing
    fn parse_block(&mut self) -> ParseResult<ASTNode> {
        self.lexer.next();
        let mut statements = vec![];
        while self.lexer.peek().token_type != TokenType::RightBrace {
            statements.push(self.parse_statement()?);
        }
        self.lexer.next(); // consume RightBrace
        Ok(ASTNode::Block(statements))
    }

    // fn parse_block(&mut self) -> ParseResult<Vec<ASTNode>> {
    //     // assert_eq!(self.lexer.next().token_type, TokenType::LeftBrace);
    //     if self.lexer.next().token_type != TokenType::LeftBrace {
    //         return Err(ParseError::MissingToken(
    //             TokenType::LeftBrace,
    //             "to start block".to_string(),
    //         ));
    //     }
    //     let statements = self.parse();
    //     // assert_eq!(self.lexer.next().token_type, TokenType::RightBrace);
    //     if self.lexer.next().token_type != TokenType::RightBrace {
    //         return Err(ParseError::MissingToken(
    //             TokenType::RightBrace,
    //             "to close block".to_string(),
    //         ));
    //     }
    //     statements
    // }

    // TODO: might need fixing
    fn parse_if(&mut self) -> ParseResult<ASTNode> {
        self.lexer.next();
        if self.lexer.next().token_type != TokenType::LeftParen {
            return Err(ParseError::MissingToken(
                TokenType::LeftParen,
                "to start if condition".to_string(),
            ));
        }
        let condition = self.parse_expression()?;
        if self.lexer.next().token_type != TokenType::RightParen {
            return Err(ParseError::MissingToken(
                TokenType::RightParen,
                "to close if condition".to_string(),
            ));
        }
        // let then_branch = self.parse_block()?;

        // let else_branch = if self.lexer.peek().token_type == TokenType::ELSE {
        //     self.lexer.next();
        //     Some(self.parse_block()?)
        // } else {
        //     None
        // };
        let then_branch = vec![self.parse_statement()?];
        let else_branch = if self.lexer.peek().token_type == TokenType::ELSE {
            self.lexer.next();
            Some(vec![self.parse_statement()?])
        } else {
            None
        };
        Ok(ASTNode::If(vec![condition], then_branch, else_branch))
    }

    // TODO: might need fixing
    fn parse_while(&mut self) -> ParseResult<ASTNode> {
        self.lexer.next();
        if self.lexer.next().token_type != TokenType::LeftParen {
            return Err(ParseError::MissingToken(
                TokenType::LeftParen,
                "to start while condition".to_string(),
            ));
        }
        let condition = self.parse_expression()?;
        if self.lexer.next().token_type != TokenType::RightParen {
            return Err(ParseError::MissingToken(
                TokenType::RightParen,
                "to close while condition".to_string(),
            ));
        }
        // let body = self.parse_block()?;
        let body = vec![self.parse_statement()?];
        Ok(ASTNode::While(vec![condition], body))
    }

    fn parse_function(&mut self) -> ParseResult<ASTNode> {
        self.lexer.next();
        let name = self.lexer.next().lexeme;
        if self.lexer.next().token_type != TokenType::LeftParen {
            return Err(ParseError::MissingToken(
                TokenType::LeftParen,
                "to start function parameters".to_string(),
            ));
        }
        let mut params = vec![];
        while self.lexer.peek().token_type != TokenType::RightParen {
            params.push(self.lexer.next().lexeme);
            if self.lexer.peek().token_type == TokenType::COMMA {
                self.lexer.next();
            }
        }
        self.lexer.next(); // consume RightParen
                           // let body = self.parse_block()?;
        let body = vec![self.parse_statement()?];
        Ok(ASTNode::Function(name, params, body))
    }
    fn parse_assign(&mut self) -> ParseResult<ASTNode> {
        let id = self.lexer.next().lexeme;
        let op = self.lexer.next().token_type;
        let expr = self.parse_expression()?;

        let expr = match op {
            TokenType::EQUAL => expr,
            TokenType::PlusEqual
            | TokenType::MinusEqual
            | TokenType::StarEqual
            | TokenType::SlashEqual => {
                let bin_op = match op {
                    TokenType::PlusEqual => BinaryOp::Add,
                    TokenType::MinusEqual => BinaryOp::Sub,
                    TokenType::StarEqual => BinaryOp::Mul,
                    TokenType::SlashEqual => BinaryOp::Div,
                    _ => {
                        return Err(ParseError::InvalidOperator(format!(
                            "Invalid assignment operator: {:?}",
                            op
                        )))
                    }
                };
                ASTNode::Op(
                    Ops::BinaryOp(bin_op),
                    vec![ASTNode::Identifier(id.clone()), expr],
                )
            }
            _ => {
                return Err(ParseError::InvalidOperator(format!(
                    "Invalid assignment operator: {:?}",
                    op
                )))
            }
        };

        Ok(ASTNode::Assign(id, vec![expr]))
    }

    /// Parse an expression using Pratt parsing
    fn parse_expression(&mut self) -> ParseResult<ASTNode> {
        expr_bp(self.lexer, 0)
    }

    // Helper methods
    fn is_assignment(&mut self) -> bool {
        let peek_types = self.lexer.peek_n_type(2);
        peek_types.contains(&TokenType::EQUAL)
            || peek_types.contains(&TokenType::PlusEqual)
            || peek_types.contains(&TokenType::MinusEqual)
            || peek_types.contains(&TokenType::StarEqual)
            || peek_types.contains(&TokenType::SlashEqual)
    }

    fn expect_token(&mut self, expected: TokenType, context: &str) -> ParseResult<()> {
        let token = self.lexer.next();
        if token.token_type != expected {
            Err(ParseError::UnexpectedToken(
                token.token_type,
                format!("Expected {:?} {}", expected, context),
            ))
        } else {
            Ok(())
        }
    }
}

/// Pratt parser for expressions
fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> ParseResult<ASTNode> {
    let mut lhs = parse_prefix(lexer)?;

    loop {
        let op = match infix_op(lexer.peek().token_type) {
            Some(op) => op,
            None => break,
        };

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp)?;
            lhs = ASTNode::Op(op, vec![lhs, rhs]);
        } else if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = parse_postfix(op, lhs, lexer)?;
        } else {
            break;
        }
    }

    Ok(lhs)
}

/// Parse prefix expressions
fn parse_prefix(lexer: &mut Lexer) -> ParseResult<ASTNode> {
    if lexer.peek().token_type == TokenType::EOF {
        return Err(ParseError::UnexpectedToken(
            TokenType::EOF,
            "Unexpected end of input".to_string(),
        ));
    }

    let token = lexer.next();
    match token.token_type {
        TokenType::IntNumber(n) => Ok(ASTNode::IntNumber(n)),
        TokenType::FloatNumber(n) => Ok(ASTNode::FloatNumber(n)),
        TokenType::Identifier => Ok(ASTNode::Identifier(token.lexeme)),
        TokenType::Boolean(b) => Ok(ASTNode::Boolean(b)),
        TokenType::String => Ok(ASTNode::String(token.lexeme)),
        TokenType::LeftParen => {
            let expr = expr_bp(lexer, 0)?;
            if lexer.next().token_type != TokenType::RightParen {
                return Err(ParseError::MissingToken(
                    TokenType::RightParen,
                    "to close parenthesized expression".to_string(),
                ));
            }
            Ok(expr)
        }
        TokenType::PLUS | TokenType::MINUS | TokenType::BANG => {
            let op = match token.token_type {
                TokenType::MINUS => Ops::UnaryOp(UnaryOp::Negate),
                TokenType::BANG => Ops::UnaryOp(UnaryOp::Not),
                _ => {
                    return Err(ParseError::InvalidOperator(format!(
                        "Invalid prefix operator: {:?}",
                        token.token_type
                    )))
                }
            };
            let ((), r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp)?;
            Ok(ASTNode::Op(op, vec![rhs]))
        }
        _ => Err(ParseError::UnexpectedToken(
            token.token_type,
            "in prefix position".to_string(),
        )),
    }
}

/// Parse postfix expressions
fn parse_postfix(op: Ops, lhs: ASTNode, lexer: &mut Lexer) -> ParseResult<ASTNode> {
    match op {
        Ops::PostfixOp(PostfixOp::Index) => {
            let rhs = expr_bp(lexer, 0)?;
            if lexer.next().token_type != TokenType::RightBracket {
                return Err(ParseError::MissingToken(
                    TokenType::RightBracket,
                    "to close index operation".to_string(),
                ));
            }
            Ok(ASTNode::Op(op, vec![lhs, rhs]))
        }
        Ops::PostfixOp(PostfixOp::Call) => {
            let callee = lexer.next().lexeme;
            if lexer.next().token_type != TokenType::LeftParen {
                return Err(ParseError::MissingToken(
                    TokenType::LeftParen,
                    "to start function call arguments".to_string(),
                ));
            }
            let args = parse_args(lexer)?;
            if lexer.next().token_type != TokenType::RightParen {
                return Err(ParseError::MissingToken(
                    TokenType::RightParen,
                    "to close function call arguments".to_string(),
                ));
            }
            Ok(ASTNode::Op(op, vec![lhs, ASTNode::Callee(callee, args)]))
        }
        Ops::PostfixOp(PostfixOp::StarStar) => {
            let rhs = expr_bp(lexer, 0)?;
            Ok(ASTNode::Op(op, vec![lhs, rhs]))
        }
        _ => Err(ParseError::InvalidOperator(format!(
            "Invalid postfix operator: {:?}",
            op
        ))),
    }
}

/// Parse function arguments
fn parse_args(lexer: &mut Lexer) -> ParseResult<Vec<ASTNode>> {
    let mut args = Vec::new();
    while lexer.peek().token_type != TokenType::RightParen {
        args.push(expr_bp(lexer, 0)?);
        if lexer.peek().token_type == TokenType::COMMA {
            lexer.next();
        }
    }
    Ok(args)
}

/// Get the infix operator from a token type
fn infix_op(token_type: TokenType) -> Option<Ops> {
    match token_type {
        TokenType::PLUS => Some(Ops::BinaryOp(BinaryOp::Add)),
        TokenType::MINUS => Some(Ops::BinaryOp(BinaryOp::Sub)),
        TokenType::STAR => Some(Ops::BinaryOp(BinaryOp::Mul)),
        TokenType::SLASH => Some(Ops::BinaryOp(BinaryOp::Div)),
        TokenType::AT => Some(Ops::BinaryOp(BinaryOp::At)),
        TokenType::EqualEqual => Some(Ops::BinaryOp(BinaryOp::Eq)),
        TokenType::BangEqual => Some(Ops::BinaryOp(BinaryOp::Ne)),
        TokenType::LESS => Some(Ops::BinaryOp(BinaryOp::Lt)),
        TokenType::LessEqual => Some(Ops::BinaryOp(BinaryOp::Le)),
        TokenType::GREATER => Some(Ops::BinaryOp(BinaryOp::Gt)),
        TokenType::GreaterEqual => Some(Ops::BinaryOp(BinaryOp::Ge)),
        TokenType::DOT => Some(Ops::PostfixOp(PostfixOp::Call)),
        TokenType::LeftBracket => Some(Ops::PostfixOp(PostfixOp::Index)),
        TokenType::StarStar => Some(Ops::PostfixOp(PostfixOp::StarStar)),
        _ => None,
    }
}

/// Get the binding power for prefix operators
fn prefix_binding_power(op: Ops) -> ((), u8) {
    match op {
        Ops::UnaryOp(UnaryOp::Not) | Ops::UnaryOp(UnaryOp::Negate) => ((), 15),
        _ => panic!("Invalid prefix operator: {:?}", op),
    }
}

/// Get the binding power for postfix operators
fn postfix_binding_power(op: Ops) -> Option<(u8, ())> {
    match op {
        Ops::PostfixOp(PostfixOp::Index) => Some((13, ())),
        Ops::PostfixOp(PostfixOp::Call) => Some((14, ())),
        Ops::PostfixOp(PostfixOp::StarStar) => Some((16, ())),
        _ => None,
    }
}

/// Get the binding power for infix operators
fn infix_binding_power(op: Ops) -> Option<(u8, u8)> {
    match op {
        Ops::BinaryOp(BinaryOp::Eq) | Ops::BinaryOp(BinaryOp::Ne) => Some((5, 6)),
        Ops::BinaryOp(BinaryOp::Lt)
        | Ops::BinaryOp(BinaryOp::Le)
        | Ops::BinaryOp(BinaryOp::Gt)
        | Ops::BinaryOp(BinaryOp::Ge) => Some((7, 8)),
        Ops::BinaryOp(BinaryOp::Add) | Ops::BinaryOp(BinaryOp::Sub) => Some((9, 10)),
        Ops::BinaryOp(BinaryOp::Mul) | Ops::BinaryOp(BinaryOp::Div) => Some((11, 12)),
        Ops::BinaryOp(BinaryOp::At) => Some((14, 13)),
        _ => None,
    }
}

////////////////////////////////////////
//////// Display for Ops & AST /////////
////////////////////////////////////////

use colored::*;

impl fmt::Display for Ops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ops::BinaryOp(BinaryOp::Add) => write!(f, "{}", "+".green()),
            Ops::BinaryOp(BinaryOp::Sub) => write!(f, "{}", "-".green()),
            Ops::BinaryOp(BinaryOp::Mul) => write!(f, "{}", "*".green()),
            Ops::BinaryOp(BinaryOp::Div) => write!(f, "{}", "/".green()),
            Ops::BinaryOp(BinaryOp::At) => write!(f, "{}", "@".green()),
            Ops::BinaryOp(BinaryOp::Eq) => write!(f, "{}", "==".green()),
            Ops::BinaryOp(BinaryOp::Ne) => write!(f, "{}", "!=".green()),
            Ops::BinaryOp(BinaryOp::Lt) => write!(f, "{}", "<".green()),
            Ops::BinaryOp(BinaryOp::Le) => write!(f, "{}", "<=".green()),
            Ops::BinaryOp(BinaryOp::Gt) => write!(f, "{}", ">".green()),
            Ops::BinaryOp(BinaryOp::Ge) => write!(f, "{}", ">=".green()),

            Ops::UnaryOp(UnaryOp::Negate) => write!(f, "{}", "-".green()),
            Ops::UnaryOp(UnaryOp::Not) => write!(f, "{}", "!".green()),

            Ops::PostfixOp(PostfixOp::Index) => write!(f, "["),
            Ops::PostfixOp(PostfixOp::Call) => write!(f, "."),
            Ops::PostfixOp(PostfixOp::StarStar) => write!(f, "**"),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ASTNode::Number(i) => write!(f, "{}", i.to_string().blue()),
            ASTNode::IntNumber(i) => write!(f, "{}", i.to_string().blue()),
            ASTNode::FloatNumber(i) => write!(f, "{}", i.to_string().blue()),
            ASTNode::Identifier(s) => write!(f, "{}", s.red()),
            ASTNode::Boolean(b) => write!(f, "{}", b.to_string().yellow()),
            ASTNode::String(s) => write!(f, "{}", s.yellow()),
            ASTNode::Callee(callee, args) => {
                write!(f, "({}", callee.purple().magenta())?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, "{}", ")".normal().clear())
            }
            ASTNode::Print(expr) => {
                write!(f, "print!(")?;
                for e in expr {
                    write!(f, "{}, ", e)?;
                }
                write!(f, ")")
            }
            ASTNode::Let(identifier, expr) => {
                write!(f, "let {} = {}", identifier, expr[0])
            }
            ASTNode::Block(statements) => {
                for stmt in statements {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "")
            }
            ASTNode::Assign(identifier, expr) => {
                write!(f, "{} = {}", identifier, expr[0])
            }
            ASTNode::If(condition, then_branch, else_branch) => {
                write!(f, "if {} {{", condition[0])?;
                for stmt in then_branch {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")?;
                if let Some(else_branch) = else_branch {
                    write!(f, " else {{")?;
                    for stmt in else_branch {
                        write!(f, "{}", stmt)?;
                    }
                    write!(f, "}}")?;
                }
                write!(f, "")
            }
            ASTNode::While(condition, body) => {
                write!(f, "while {} {{", condition[0])?;
                for stmt in body {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
            ASTNode::Op(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
            ASTNode::Function(name, params, body) => {
                write!(f, "fn {}(", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") {{")?;
                for stmt in body {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
        }
    }
}

use std::fmt::Write;

pub fn ast_to_ascii(node: &ASTNode, indent: usize) -> String {
    let mut result = String::new();
    let indent_str = "  ".repeat(indent);

    match node {
        ASTNode::IntNumber(n) => writeln!(result, "{}IntNumber({})", indent_str, n).unwrap(),
        ASTNode::FloatNumber(n) => writeln!(result, "{}FloatNumber({})", indent_str, n).unwrap(),
        ASTNode::Identifier(s) => writeln!(result, "{}Identifier({})", indent_str, s).unwrap(),
        ASTNode::Boolean(b) => writeln!(result, "{}Boolean({})", indent_str, b).unwrap(),
        ASTNode::String(s) => writeln!(result, "{}String(\"{}\")", indent_str, s).unwrap(),
        ASTNode::Op(op, args) => {
            writeln!(result, "{}Op({:?})", indent_str, op).unwrap();
            for arg in args {
                result.push_str(&ast_to_ascii(arg, indent + 1));
            }
        }
        ASTNode::Callee(name, args) => {
            writeln!(result, "{}Callee({})", indent_str, name).unwrap();
            for arg in args {
                result.push_str(&ast_to_ascii(arg, indent + 1));
            }
        }
        ASTNode::Let(name, value) => {
            writeln!(result, "{}Let({})", indent_str, name).unwrap();
            for v in value {
                result.push_str(&ast_to_ascii(v, indent + 1));
            }
        }
        ASTNode::Assign(name, value) => {
            writeln!(result, "{}Assign({})", indent_str, name).unwrap();
            for v in value {
                result.push_str(&ast_to_ascii(v, indent + 1));
            }
        }
        ASTNode::If(condition, then_branch, else_branch) => {
            writeln!(result, "{}If", indent_str).unwrap();
            writeln!(result, "{}  Condition:", indent_str).unwrap();
            for cond in condition {
                result.push_str(&ast_to_ascii(cond, indent + 2));
            }
            writeln!(result, "{}  Then:", indent_str).unwrap();
            for stmt in then_branch {
                result.push_str(&ast_to_ascii(stmt, indent + 2));
            }
            if let Some(else_branch) = else_branch {
                writeln!(result, "{}  Else:", indent_str).unwrap();
                for stmt in else_branch {
                    result.push_str(&ast_to_ascii(stmt, indent + 2));
                }
            }
        }
        ASTNode::While(condition, body) => {
            writeln!(result, "{}While", indent_str).unwrap();
            writeln!(result, "{}  Condition:", indent_str).unwrap();
            for cond in condition {
                result.push_str(&ast_to_ascii(cond, indent + 2));
            }
            writeln!(result, "{}  Body:", indent_str).unwrap();
            for stmt in body {
                result.push_str(&ast_to_ascii(stmt, indent + 2));
            }
        }
        ASTNode::Print(args) => {
            writeln!(result, "{}Print", indent_str).unwrap();
            for arg in args {
                result.push_str(&ast_to_ascii(arg, indent + 1));
            }
        }
        ASTNode::Function(name, params, body) => {
            writeln!(result, "{}Function({})", indent_str, name).unwrap();
            writeln!(result, "{}  Parameters: {:?}", indent_str, params).unwrap();
            writeln!(result, "{}  Body:", indent_str).unwrap();
            for stmt in body {
                result.push_str(&ast_to_ascii(stmt, indent + 2));
            }
        }
        ASTNode::Block(statements) => {
            writeln!(result, "{}Block", indent_str).unwrap();
            for stmt in statements {
                result.push_str(&ast_to_ascii(stmt, indent + 1));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        fn expr(source: &str) -> String {
            let mut lexer = Lexer::new(source.to_string());
            format!("{}", expr_bp(&mut lexer, 0).unwrap())
        }

        let s = expr("1");
        assert_eq!(s, "1");

        let s = expr("1 + 2 * 3");
        assert_eq!(s, "(+ 1 (* 2 3))");

        let s = expr("(1 + 2) * 3");
        assert_eq!(s, "(* (+ 1 2) 3)");

        let s = expr("a + b * c * d + e");
        assert_eq!(s, "(+ (+ a (* (* b c) d)) e)");

        let s = expr("a + b * c * d + e");
        assert_eq!(s, "(+ (+ a (* (* b c) d)) e)");

        let s = expr("f @ g @ h");
        assert_eq!(s, "(@ f (@ g h))");

        let s = expr("1 + 2 + f @ g @ h * 3 * 4");
        assert_eq!(s, "(+ (+ 1 2) (* (* (@ f (@ g h)) 3) 4))");

        let s = expr("--1 * 2");
        assert_eq!(s, "(* (- -1) 2)");

        let s = expr("--f @ g");
        assert_eq!(s, "(@ (- (- f)) g)");

        let s = expr("-!9");
        assert_eq!(s, "(- (! 9))");

        let s = expr("! f @ g ");
        assert_eq!(s, "(@ (! f) g)");

        let s = expr("(((0)))");
        assert_eq!(s, "0");

        let s = expr("x[0][1]");
        assert_eq!(s, "([ ([ x 0) 1)");

        let s = expr("x.relu()");
        assert_eq!(s, "(. x (relu))");

        let s = expr("x.relu(0, 1).relu(2, 3)");
        assert_eq!(s, "(. (. x (relu 0 1)) (relu 2 3))");

        let s = expr("x.relu(a.b(0+2), 2-1).max(0)/2");
        assert_eq!(s, "(/ (. (. x (relu (. a (b (+ 0 2))) 2 -1)) (max 0)) 2)");

        let s = expr("x.relu(a.sigmoid(0+2))");
        assert_eq!(s, "(. x (relu (. a (sigmoid (+ 0 2)))))");

        let s = expr("a == b");
        assert_eq!(s, "(== a b)");

        let s = expr("--1");
        assert_eq!(s, "(- -1)");
    }

    #[test]
    fn test_parser() {
        fn parse(source: &str) -> String {
            let mut lexer = Lexer::new(source.to_string());
            let out = Parser::new(&mut lexer).parse().unwrap();
            assert!(out.len() == 1);
            format!("{}", out[0])
        }

        // let declaration tests; let a= 3;
        let s = parse("let a = 3;");
        assert_eq!(s, "let a = 3");

        // assignment tests; a = 3, a+=3, a-=4, a*=5, a/=6
        let s = parse("a += 3;");
        assert_eq!(s, "a = (+ a 3)");

        let s = parse("a += (c == 4);");
        assert_eq!(s, "a = (+ a (== c 4))");

        let s = parse("a -= 4;");
        assert_eq!(s, "a = (- a 4)");

        let s = parse("a *= (5 != 4);");
        assert_eq!(s, "a = (* a (!= 5 4))");

        // function definition test
        let s = parse("fn add(a, b) { a + b; }");
        assert_eq!(s, "fn add(a, b) {(+ a b)}");
    }
}
