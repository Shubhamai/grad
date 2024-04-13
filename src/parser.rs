use crate::lexer::{Span, Token};
use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Tensor(f64),
    Null,
}

#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub msg: String,
}

impl<'src> Value {
    fn tensor(self, span: Span) -> Result<f64, Error> {
        if let Value::Tensor(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a number", self),
                // msg: format!("is not a number"),
            })
        }
    }
}



impl<'src> std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Tensor(x) => write!(f, "{}", x),
           
        }
    }
}




pub type Spanned<T> = (T, Span);

// An expression node in the AST. Children are spanned so we can generate useful runtime errors.
#[derive(Debug)]
pub enum Expr<'src> {
    Error,
    Value(Value),

    Local(&'src str),
    Let(&'src str, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Then(Box<Spanned<Self>>, Box<Spanned<Self>>),
    
    Print(Box<Spanned<Self>>),
}

type ParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<Token<'src>, Span, &'tokens [(Token<'src>, Span)]>;

pub fn expr_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<Expr<'src>>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    recursive(|expr| {
        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Tensor(n) => Expr::Value(Value::Tensor(n)),
            }
            .labelled("value");

            let ident = select! { Token::Ident(ident) => ident }.labelled("identifier");

            // A let expression
            let let_ = just(Token::Let)
                .ignore_then(ident)
                .then_ignore(just(Token::BinaryOps("=")))
                .then(inline_expr)
                .then_ignore(just(Token::Ctrl(';')))
                .then(expr.clone())
                .map(|((name, val), body)| Expr::Let(name, Box::new(val), Box::new(body)));

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(ident.map(Expr::Local))
                .or(let_)
                .or(just(Token::Print)
                    .ignore_then(
                        expr.clone()
                            .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
                    )
                    .map(|expr| Expr::Print(Box::new(expr))))
                .map_with(|expr, e| (expr, e.span()))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))));

            atom
        });

        // inline_expr

               // Blocks are expressions but delimited with braces
               let block = expr
               .clone()
               .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
               // Attempt to recover anything that looks like a block but contains errors
               .recover_with(via_parser(nested_delimiters(
                   Token::Ctrl('{'),
                   Token::Ctrl('}'),
                   [
                       (Token::Ctrl('('), Token::Ctrl(')')),
                       (Token::Ctrl('['), Token::Ctrl(']')),
                   ],
                   |span| (Expr::Error, span),
               )));

           // Both blocks and `if` are 'block expressions' and can appear in the place of statements
           let block_expr = block;

           let block_chain = block_expr
               .clone()
               .foldl_with(block_expr.clone().repeated(), |a, b, e| {
                   (Expr::Then(Box::new(a), Box::new(b)), e.span())
               });

           let block_recovery = nested_delimiters(
               Token::Ctrl('{'),
               Token::Ctrl('}'),
               [
                   (Token::Ctrl('('), Token::Ctrl(')')),
                   (Token::Ctrl('['), Token::Ctrl(']')),
               ],
               |span| (Expr::Error, span),
           );

           block_chain
               .labelled("block")
               // Expressions, chained by semicolons, are statements
               .or(inline_expr.clone())
               .recover_with(skip_then_retry_until(
                   block_recovery.ignored().or(any().ignored()),
                   one_of([
                       Token::Ctrl(';'),
                       Token::Ctrl('}'),
                       Token::Ctrl(')'),
                       Token::Ctrl(']'),
                   ])
                   .ignored(),
               ))
               .foldl_with(
                   just(Token::Ctrl(';')).ignore_then(expr.or_not()).repeated(),
                   |a, b, e| {
                       let span: Span = e.span();
                       (
                           Expr::Then(
                               Box::new(a),
                               // If there is no b expression then its span is the end of the statement/block.
                               Box::new(
                                   b.unwrap_or_else(|| (Expr::Value(Value::Null), span.to_end())),
                               ),
                           ),
                           span,
                       )
                   },
               )
    })
}
