// Pratt parser for parsing expressions from https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

use crate::{
    scanner::{Lexer, TokenType},
    vm,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Identifier(String),
    Boolean(bool),
    String(String),
    Op(Ops, Vec<ASTNode>),
    Callee(String, Vec<ASTNode>), // function call with arguments
    Let(String, Vec<ASTNode>),
    Assign(String, Vec<ASTNode>),
    If(Vec<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>), // condition, then, else
    While(Vec<ASTNode>, Vec<ASTNode>),                    // condition, body
    Print(Vec<ASTNode>),
    Block(Vec<ASTNode>), // depth, statements
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    At, // dot product

    // comparison
    Eq, // ==
    Ne, // !=
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not, // ! - logical not
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOp {
    Index,
    Call,
    StarStar, // exponentiation
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ops {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    PostfixOp(PostfixOp),
}

//////////////////////////////
/// Parser
//////////////////////////////

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        Parser { lexer }
    }
    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut statements = vec![];

        let mut all_errors = vec![];

        while self.lexer.peek().token_type != TokenType::EOF {
            let statement = match self.lexer.peek().token_type {
                TokenType::PRINT => self.parse_print(),
                TokenType::LET => self.parse_let(),
                TokenType::LeftBrace => {
                    self.lexer.next();
                    let statements = self.parse();
                    assert_eq!(self.lexer.next().token_type, TokenType::RightBrace);
                    ASTNode::Block(statements)
                }
                TokenType::RightBrace => break,
                // contains equal pr +=, -=, *=, /=
                TokenType::Identifier
                    if self.lexer.peek_n_type(2).contains(&TokenType::EQUAL)
                        | self.lexer.peek_n_type(2).contains(&TokenType::PlusEqual)
                        || self.lexer.peek_n_type(2).contains(&TokenType::MinusEqual)
                        || self.lexer.peek_n_type(2).contains(&TokenType::StarEqual)
                        || self.lexer.peek_n_type(2).contains(&TokenType::SlashEqual) =>
                {
                    // self.parse_assign()
                    match self.parse_assign() {
                        Ok(ast) => ast,
                        Err(e) => {
                            all_errors.push(e);
                            self.lexer.next();
                            continue;
                        }
                    }
                }
                TokenType::IF => {
                    self.lexer.next();
                    assert_eq!(self.lexer.next().token_type, TokenType::LeftParen);
                    let condition = self.parse_expression();
                    assert_eq!(self.lexer.next().token_type, TokenType::RightParen);
                    let then_branch = self.parse_block();
                    let else_branch = if self.lexer.peek().token_type == TokenType::ELSE {
                        self.lexer.next();
                        Some(self.parse_block())
                    } else {
                        None
                    };
                    ASTNode::If(vec![condition], then_branch, else_branch)
                }
                TokenType::WHILE => {
                    self.lexer.next();
                    assert_eq!(self.lexer.next().token_type, TokenType::LeftParen);
                    let condition = self.parse_expression();
                    assert_eq!(self.lexer.next().token_type, TokenType::RightParen);
                    let body = self.parse_block();
                    ASTNode::While(vec![condition], body)
                }
                TokenType::SEMICOLON => {
                    self.lexer.next();
                    continue;
                }
                _ => self.parse_expression(),
            };
            statements.push(statement);
        }

        statements
    }

    fn parse_print(&mut self) -> ASTNode {
        self.lexer.next();
        assert_eq!(self.lexer.next().token_type, TokenType::LeftParen);
        let expr = self.parse_expression();
        assert_eq!(self.lexer.next().token_type, TokenType::RightParen);
        ASTNode::Print(vec![expr])
    }

    fn parse_let(&mut self) -> ASTNode {
        self.lexer.next();
        let identifier = self.lexer.next().lexeme;
        assert_eq!(self.lexer.next().token_type, TokenType::EQUAL);
        let expr = self.parse_expression();
        ASTNode::Let(identifier, vec![expr])
    }

    fn parse_assign(&mut self) -> Result<ASTNode, vm::Result> {
        let identifier = self.lexer.next().lexeme;
        let op_lexer = self.lexer.next();
        let op = op_lexer.token_type;
        let expr = self.parse_expression();
        let expr = if op == TokenType::EQUAL {
            expr
        } else {
            let op = match op {
                TokenType::PlusEqual => Ops::BinaryOp(BinaryOp::Add),
                TokenType::MinusEqual => Ops::BinaryOp(BinaryOp::Sub),
                TokenType::StarEqual => Ops::BinaryOp(BinaryOp::Mul),
                TokenType::SlashEqual => Ops::BinaryOp(BinaryOp::Div),
                _ => {
                    return Err(vm::Result::CompileErr(format!(
                        "Invalid operator: {:?} on line: {:?}",
                        op, op_lexer.span
                    )))
                }
            };
            ASTNode::Op(op, vec![ASTNode::Identifier(identifier.clone()), expr])
        };
        Ok(ASTNode::Assign(identifier, vec![expr]))
    }

    fn parse_block(&mut self) -> Vec<ASTNode> {
        assert_eq!(self.lexer.next().token_type, TokenType::LeftBrace);
        let statements = self.parse();
        assert_eq!(self.lexer.next().token_type, TokenType::RightBrace);
        statements
    }

    fn parse_expression(&mut self) -> ASTNode {
        expr_bp(self.lexer, 0)
    }
}

////////////////////////////// Pratt Parser //////////////////////////////
/// Pratt parser for parsing expressions from https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
//////////////////////////////////////////////////////////////////////////

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> ASTNode {
    let current_token = lexer.next();

    let mut lhs = match current_token.token_type {
        TokenType::Number(it) => ASTNode::Number(it),
        TokenType::Identifier => ASTNode::Identifier(current_token.lexeme),
        TokenType::Boolean(it) => ASTNode::Boolean(it),
        TokenType::String => ASTNode::String(current_token.lexeme),
        TokenType::LeftParen => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next().token_type, TokenType::RightParen);
            lhs
        }
        TokenType::PLUS
        | TokenType::MINUS
        | TokenType::STAR
        | TokenType::SLASH
        | TokenType::AT
        | TokenType::EqualEqual
        | TokenType::BangEqual
        | TokenType::LESS
        | TokenType::LessEqual
        | TokenType::GREATER
        | TokenType::GreaterEqual
        | TokenType::DOT
        | TokenType::StarStar
        | TokenType::BANG => {
            let op = match current_token.token_type {
                TokenType::PLUS => Ops::BinaryOp(BinaryOp::Add),
                TokenType::MINUS => Ops::UnaryOp(UnaryOp::Negate),
                TokenType::STAR => Ops::BinaryOp(BinaryOp::Mul),
                TokenType::SLASH => Ops::BinaryOp(BinaryOp::Div),
                TokenType::AT => Ops::BinaryOp(BinaryOp::At),

                TokenType::EqualEqual => Ops::BinaryOp(BinaryOp::Eq),
                TokenType::BangEqual => Ops::BinaryOp(BinaryOp::Ne),
                TokenType::LESS => Ops::BinaryOp(BinaryOp::Lt),
                TokenType::LessEqual => Ops::BinaryOp(BinaryOp::Le),
                TokenType::GREATER => Ops::BinaryOp(BinaryOp::Gt),
                TokenType::GreaterEqual => Ops::BinaryOp(BinaryOp::Ge),

                TokenType::DOT => Ops::PostfixOp(PostfixOp::Call),
                TokenType::StarStar => Ops::PostfixOp(PostfixOp::StarStar),

                TokenType::BANG => Ops::UnaryOp(UnaryOp::Not),

                _ => unreachable!(),
            };

            let ((), r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp);
            ASTNode::Op(op, vec![rhs])
        }
        t => Err(vm::Result::CompileErr(format!("bad token: {:?}", t))).unwrap(),
    };

    loop {
        let op = match lexer.peek().token_type {
            TokenType::EOF => break,
            TokenType::PLUS => Ops::BinaryOp(BinaryOp::Add),
            TokenType::MINUS => Ops::BinaryOp(BinaryOp::Sub),
            TokenType::STAR => Ops::BinaryOp(BinaryOp::Mul),
            TokenType::SLASH => Ops::BinaryOp(BinaryOp::Div),
            TokenType::AT => Ops::BinaryOp(BinaryOp::At),

            TokenType::EqualEqual => Ops::BinaryOp(BinaryOp::Eq),
            TokenType::BangEqual => Ops::BinaryOp(BinaryOp::Ne),
            TokenType::LESS => Ops::BinaryOp(BinaryOp::Lt),
            TokenType::LessEqual => Ops::BinaryOp(BinaryOp::Le),
            TokenType::GREATER => Ops::BinaryOp(BinaryOp::Gt),
            TokenType::GreaterEqual => Ops::BinaryOp(BinaryOp::Ge),

            TokenType::DOT => Ops::PostfixOp(PostfixOp::Call),
            TokenType::LeftBracket => Ops::PostfixOp(PostfixOp::Index),
            TokenType::StarStar => Ops::PostfixOp(PostfixOp::StarStar),

            TokenType::BANG => Ops::UnaryOp(UnaryOp::Negate),

            TokenType::LeftParen => break,
            TokenType::RightParen => break,
            TokenType::RightBracket => break,
            TokenType::COMMA => break,
            TokenType::SEMICOLON => break,
            t => Err(vm::Result::CompileErr(format!("bad token: {:?}", t))).unwrap(),
        };

        if let Some((l_bp, _)) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op == Ops::PostfixOp(PostfixOp::Index) {
                let rhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next().token_type, TokenType::RightBracket);
                ASTNode::Op(op, vec![lhs, rhs])
            } else if op == Ops::PostfixOp(PostfixOp::Call) {
                let callee_token = lexer.next();
                assert_eq!(callee_token.token_type, TokenType::Identifier);

                assert_eq!(lexer.next().token_type, TokenType::LeftParen);

                let mut args = Vec::new();
                while lexer.peek().token_type != TokenType::RightParen {
                    args.push(expr_bp(lexer, 0));
                    if lexer.peek().token_type == TokenType::COMMA {
                        lexer.next();
                    }
                }

                lexer.next();
                ASTNode::Op(op, vec![lhs, ASTNode::Callee(callee_token.lexeme, args)])
            } else if op == Ops::PostfixOp(PostfixOp::StarStar) {
                let rhs = expr_bp(lexer, 0);
                ASTNode::Op(op, vec![lhs, rhs])
            } else {
                ASTNode::Op(op, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            // lhs =
            // if op == '?' {
            //     let mhs = expr_bp(lexer, 0);
            //     assert_eq!(lexer.next(), Token::Op(':'));
            //     let rhs = expr_bp(lexer, r_bp);
            //     S::Cons(op, vec![lhs, mhs, rhs])
            // } else {
            //     let rhs = expr_bp(lexer, r_bp);
            //     S::Cons(op, vec![lhs, rhs])
            // };
            let rhs = expr_bp(lexer, r_bp);
            lhs = ASTNode::Op(op, vec![lhs, rhs]);
            continue;
        }

        break;
    }

    lhs
}

fn prefix_binding_power(op: Ops) -> ((), u8) {
    match op {
        Ops::UnaryOp(UnaryOp::Not) | Ops::UnaryOp(UnaryOp::Negate) => ((), 15),
        _ => Err(vm::Result::CompileErr(format!("bad token: {:?}", op))).unwrap(),
    }
}

fn postfix_binding_power(op: Ops) -> Option<(u8, ())> {
    let res = match op {
        Ops::PostfixOp(PostfixOp::Index) => (13, ()),
        Ops::PostfixOp(PostfixOp::Call) => (14, ()),
        Ops::PostfixOp(PostfixOp::StarStar) => (16, ()),
        _ => return None,
    };
    Some(res)
}

fn infix_binding_power(op: Ops) -> Option<(u8, u8)> {
    let res = match op {
        Ops::BinaryOp(BinaryOp::Eq) | Ops::BinaryOp(BinaryOp::Ne) => (5, 6),
        Ops::BinaryOp(BinaryOp::Lt)
        | Ops::BinaryOp(BinaryOp::Le)
        | Ops::BinaryOp(BinaryOp::Gt)
        | Ops::BinaryOp(BinaryOp::Ge) => (7, 8),
        Ops::BinaryOp(BinaryOp::Add) | Ops::BinaryOp(BinaryOp::Sub) => (9, 10),
        Ops::BinaryOp(BinaryOp::Mul) | Ops::BinaryOp(BinaryOp::Div) => (11, 12),
        Ops::BinaryOp(BinaryOp::At) => (14, 13),

        _ => return None,
    };
    Some(res)
}

////////////////////////////////////////
//////// Display for Ops & AST /////////
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
            // Ops::PostfixOp(PostfixOp::Args) => write!(f, ","),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Number(i) => write!(f, "{}", i.to_string().blue()),
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
                if !else_branch.is_none() {
                    write!(f, " else {{")?;
                    for stmt in else_branch {
                        write!(f, "{:?}", stmt)?;
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        fn expr(source: &str) -> String {
            let mut lexer = Lexer::new(source.to_string());
            format!("{}", expr_bp(&mut lexer, 0))
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
        assert_eq!(s, "(* (- (- 1)) 2)");

        let s = expr("--f @ g");
        assert_eq!(s, "(@ (- (- f)) g)");

        // let s = expr(r""sfsad"+"sdf"--4");
        // assert_eq!(s, "(+ \"sfsad\" \"sdf\" (- (- 4))");

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
        assert_eq!(
            s,
            "(/ (. (. x (relu (. a (b (+ 0 2))) (- 2 1))) (max 0)) 2)"
        );

        let s = expr("x.relu(a.sigmoid(0+2))");
        assert_eq!(s, "(. x (relu (. a (sigmoid (+ 0 2)))))");

        let s = expr("a == b");
        assert_eq!(s, "(== a b)");

        let s = expr("--1");
        assert_eq!(s, "(- (- 1))");

        // let s = expr(
        //     "a ? b :
        //      c ? d
        //      : e",
        // );
        // assert_eq!(s, "(? a b (? c d e))");

        // let s = expr("a = 0 ? b : c = d");
        // assert_eq!(s, "(= a (= (? 0 b c) d))")
    }
    #[test]
    fn test_parser() {
        fn parse(source: &str) -> String {
            let mut lexer = Lexer::new(source.to_string());
            let out = Parser::new(&mut lexer).parse();
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
    }
}
