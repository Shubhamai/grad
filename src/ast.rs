// Pratt parser for parsing expressions from https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

use crate::scanner::{Lexer, TokenType};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Identifier(String),
    Boolean(bool),
    String(String),
    Op(Ops, Vec<ASTNode>),
    Callee(String, Vec<ASTNode>), // function call with arguments
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
    Not, // ! - logical not
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOp {
    Index,
    Call,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ops {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    PostfixOp(PostfixOp),
}

impl fmt::Display for Ops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ops::BinaryOp(BinaryOp::Add) => write!(f, "+"),
            Ops::BinaryOp(BinaryOp::Sub) => write!(f, "-"),
            Ops::BinaryOp(BinaryOp::Mul) => write!(f, "*"),
            Ops::BinaryOp(BinaryOp::Div) => write!(f, "/"),
            Ops::BinaryOp(BinaryOp::At) => write!(f, "@"),
            Ops::BinaryOp(BinaryOp::Eq) => write!(f, "=="),
            Ops::BinaryOp(BinaryOp::Ne) => write!(f, "!="),
            Ops::BinaryOp(BinaryOp::Lt) => write!(f, "<"),
            Ops::BinaryOp(BinaryOp::Le) => write!(f, "<="),
            Ops::BinaryOp(BinaryOp::Gt) => write!(f, ">"),
            Ops::BinaryOp(BinaryOp::Ge) => write!(f, ">="),

            Ops::UnaryOp(UnaryOp::Not) => write!(f, "!"),

            Ops::PostfixOp(PostfixOp::Index) => write!(f, "["),
            Ops::PostfixOp(PostfixOp::Call) => write!(f, "."),
            // Ops::PostfixOp(PostfixOp::Args) => write!(f, ","),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Number(i) => write!(f, "{}", i),
            ASTNode::Identifier(s) => write!(f, "{}", s),
            ASTNode::Boolean(b) => write!(f, "{}", b),
            ASTNode::String(s) => write!(f, "{}", s),
            ASTNode::Callee(callee, args) => {
                write!(f, "({}", callee)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
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

pub fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> ASTNode {
    let current_token = lexer.next();

    let mut lhs = match current_token.token_type {
        TokenType::Number(it) => ASTNode::Number(it),
        TokenType::Identifier => ASTNode::Identifier(current_token.lexeme),
        TokenType::Boolean(it) => ASTNode::Boolean(it),
        TokenType::String => ASTNode::String(current_token.lexeme),
        TokenType::LEFT_PAREN => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next().token_type, TokenType::RIGHT_PAREN);
            lhs
        }
        TokenType::PLUS
        | TokenType::MINUS
        | TokenType::STAR
        | TokenType::SLASH
        | TokenType::AT
        | TokenType::EQUAL_EQUAL
        | TokenType::BANG_EQUAL
        | TokenType::LESS
        | TokenType::LESS_EQUAL
        | TokenType::GREATER
        | TokenType::GREATER_EQUAL
        | TokenType::DOT
        | TokenType::BANG => {
            let op = match current_token.token_type {
                TokenType::PLUS => Ops::BinaryOp(BinaryOp::Add),
                TokenType::MINUS => Ops::BinaryOp(BinaryOp::Sub),
                TokenType::STAR => Ops::BinaryOp(BinaryOp::Mul),
                TokenType::SLASH => Ops::BinaryOp(BinaryOp::Div),
                TokenType::AT => Ops::BinaryOp(BinaryOp::At),

                TokenType::EQUAL_EQUAL => Ops::BinaryOp(BinaryOp::Eq),
                TokenType::BANG_EQUAL => Ops::BinaryOp(BinaryOp::Ne),
                TokenType::LESS => Ops::BinaryOp(BinaryOp::Lt),
                TokenType::LESS_EQUAL => Ops::BinaryOp(BinaryOp::Le),
                TokenType::GREATER => Ops::BinaryOp(BinaryOp::Gt),
                TokenType::GREATER_EQUAL => Ops::BinaryOp(BinaryOp::Ge),

                TokenType::DOT => Ops::PostfixOp(PostfixOp::Call),
                TokenType::BANG => Ops::UnaryOp(UnaryOp::Not),

                _ => unreachable!(),
            };

            let ((), r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp);
            // print!("{} ", op);
            ASTNode::Op(op, vec![rhs])
        }
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek().token_type {
            TokenType::EOF => break,
            TokenType::PLUS => Ops::BinaryOp(BinaryOp::Add),
            TokenType::MINUS => Ops::BinaryOp(BinaryOp::Sub),
            TokenType::STAR => Ops::BinaryOp(BinaryOp::Mul),
            TokenType::SLASH => Ops::BinaryOp(BinaryOp::Div),
            TokenType::AT => Ops::BinaryOp(BinaryOp::At),

            TokenType::EQUAL_EQUAL => Ops::BinaryOp(BinaryOp::Eq),
            TokenType::BANG_EQUAL => Ops::BinaryOp(BinaryOp::Ne),
            TokenType::LESS => Ops::BinaryOp(BinaryOp::Lt),
            TokenType::LESS_EQUAL => Ops::BinaryOp(BinaryOp::Le),
            TokenType::GREATER => Ops::BinaryOp(BinaryOp::Gt),
            TokenType::GREATER_EQUAL => Ops::BinaryOp(BinaryOp::Ge),

            TokenType::DOT => Ops::PostfixOp(PostfixOp::Call),
            TokenType::BANG => Ops::UnaryOp(UnaryOp::Not),
            TokenType::LEFT_PAREN => break,
            TokenType::RIGHT_PAREN => break,
            TokenType::LEFT_BRACKET => Ops::PostfixOp(PostfixOp::Index),
            TokenType::RIGHT_BRACKET => break,

            TokenType::COMMA => break,
            t => panic!("bad token: {:?}", t),
        };

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op == Ops::PostfixOp(PostfixOp::Index) {
                let rhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next().token_type, TokenType::RIGHT_BRACKET);
                ASTNode::Op(op, vec![lhs, rhs])
            } else if op == Ops::PostfixOp(PostfixOp::Call) {
                // identifier
                let callee_token = lexer.next();
                assert_eq!(callee_token.token_type, TokenType::Identifier);

                // left paren
                assert_eq!(lexer.next().token_type, TokenType::LEFT_PAREN);

                // a, b, c
                let mut args = Vec::new();
                while lexer.peek().token_type != TokenType::RIGHT_PAREN {
                    args.push(expr_bp(lexer, 0));
                    if lexer.peek().token_type == TokenType::COMMA {
                        lexer.next();
                    }
                }

                // let mut args = HashMap::new();
                // while lexer.peek().token_type != TokenType::RIGHT_PAREN {

                //     // let key = lexer.next().lexeme;
                //     // assert_eq!(lexer.next().token_type, TokenType::EQUAL);
                //     // let value = expr_bp(lexer, 0);
                //     // args.insert(key, value);

                //     if lexer.peek().token_type == TokenType::COMMA {
                //         lexer.next();
                //     }
                // }

                lexer.next();
                ASTNode::Op(
                    op,
                    // vec![
                    //     lhs,
                    //     S::Identifier(callee_token.lexeme),
                    //     S::Op(Ops::PostfixOp(PostfixOp::Args), args),
                    // ],
                    vec![lhs, ASTNode::Callee(callee_token.lexeme, args)],
                )
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
        Ops::BinaryOp(BinaryOp::Add)
        | Ops::BinaryOp(BinaryOp::Sub)
        | Ops::UnaryOp(UnaryOp::Not) => ((), 15),
        _ => panic!("bad op: {:?}", op),
    }
}

fn postfix_binding_power(op: Ops) -> Option<(u8, ())> {
    let res = match op {
        // '!' => (11, ()),
        // '[' => (11, ()),
        Ops::PostfixOp(PostfixOp::Index) => (13, ()),
        Ops::PostfixOp(PostfixOp::Call) => (14, ()),
        _ => return None,
    };
    Some(res)
}

fn infix_binding_power(op: Ops) -> Option<(u8, u8)> {
    let res = match op {
        // '=' => (2, 1),
        // '?' => (4, 3),

        // Token::Or => (1, 2),
        // Token::And => (3, 4),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        fn expr(source: &str) -> ASTNode {
            let mut lexer = Lexer::new(source.to_string());
            expr_bp(&mut lexer, 0)
        }

        let s = expr("1");
        assert_eq!(s.to_string(), "1");

        let s = expr("1 + 2 * 3");
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

        let s = expr("(1 + 2) * 3");
        assert_eq!(s.to_string(), "(* (+ 1 2) 3)");

        let s = expr("a + b * c * d + e");
        assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

        let s = expr("a + b * c * d + e");
        assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

        let s = expr("f @ g @ h");
        assert_eq!(s.to_string(), "(@ f (@ g h))");

        let s = expr("1 + 2 + f @ g @ h * 3 * 4");
        assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (@ f (@ g h)) 3) 4))");

        let s = expr("--1 * 2");
        assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

        let s = expr("--f @ g");
        assert_eq!(s.to_string(), "(@ (- (- f)) g)");

        // let s = expr(r""sfsad"+"sdf"--4");
        // assert_eq!(s.to_string(), "(+ \"sfsad\" \"sdf\" (- (- 4))");

        let s = expr("-!9");
        assert_eq!(s.to_string(), "(- (! 9))");

        let s = expr("! f @ g ");
        assert_eq!(s.to_string(), "(@ (! f) g)");

        let s = expr("(((0)))");
        assert_eq!(s.to_string(), "0");

        let s = expr("x[0][1]");
        assert_eq!(s.to_string(), "([ ([ x 0) 1)");

        let s = expr("x.relu()");
        assert_eq!(s.to_string(), "(. x (relu))");

        let s = expr("x.relu(0, 1).relu(2, 3)");
        assert_eq!(s.to_string(), "(. (. x (relu 0 1)) (relu 2 3))");

        let s = expr("x.relu(a.b(0+2), 2-1).max(0)/2");
        assert_eq!(
            s.to_string(),
            "(/ (. (. x (relu (. a (b (+ 0 2))) (- 2 1))) (max 0)) 2)"
        );

        // let s = expr(
        //     "a ? b :
        //      c ? d
        //      : e",
        // );
        // assert_eq!(s.to_string(), "(? a b (? c d e))");

        // let s = expr("a = 0 ? b : c = d");
        // assert_eq!(s.to_string(), "(= a (= (? 0 b c) d))")
    }
}
