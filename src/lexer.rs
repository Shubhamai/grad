use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'src> {
    Tensor(f64),

    BinaryOps(&'src str),

    Ctrl(char),
    Ident(&'src str),

    Let,
    Print
}

pub type Span = SimpleSpan<usize>;
pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Rich<'src, char, Span>>> {
    let num = just('-')
        .or_not()
        .then(text::digits(10))
        .then(just('.').then(text::digits(10)).or_not())
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Tensor);

    // A parser for operators
    let binary_ops = one_of("+*-/!=")
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::BinaryOps);

    // A parser for control characters (delimiters, semicolons, etc.)
    let ctrl = one_of("()[]{};,").map(Token::Ctrl);

    // A parser for identifiers and keywords
    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "let" => Token::Let,
        "print" => Token::Print,
        _ => Token::Ident(ident),
    });

    // A single token can be one of the above
    let token = num.or(binary_ops).or(ctrl).or(ident);

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
