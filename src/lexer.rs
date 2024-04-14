use logos::Logos;

use crate::expr::ValueType;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum TokenType {
    // Single-character tokens.
    #[token("(")]
    LEFT_PAREN,

    #[token(")")]
    RIGHT_PAREN,

    #[token("{")]
    LEFT_BRACE,

    #[token("}")]
    RIGHT_BRACE,

    #[token(",")]
    COMMA,

    #[token(".")]
    DOT,

    #[token("-")]
    MINUS,

    #[token("+")]
    PLUS,

    #[token("^")]
    HAT, // ^

    #[token(";")]
    SEMICOLON,

    #[token("/")]
    SLASH,

    #[token("*")]
    STAR,

    // One or two character tokens.
    #[token("!")]
    BANG,

    #[token("!=")]
    BANG_EQUAL,

    #[token("=")]
    EQUAL,

    #[token("==")]
    EQUAL_EQUAL,

    #[token(">")]
    GREATER,

    #[token(">=")]
    GREATER_EQUAL,

    #[token("<")]
    LESS,

    #[token("<=")]
    LESS_EQUAL,

    #[token("+=")]
    PLUS_EQUAL, // +=

    #[token("-=")]
    MINUS_EQUAL, // -=

    #[token("*=")]
    STAR_EQUAL, // *=

    #[token("/=")]
    SLASH_EQUAL, // /=

    // Literals.
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_]*"#, |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?")] // , |lex| lex.slice().parse::<f64>().unwrap())]
    Number, //(f64),

    // seperate for int and float
    // #[regex(r"-?(?:0|[1-9]\d*)", |lex| lex.slice().parse::<i64>().unwrap())]
    // IntNumber(i64),

    // #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap(), priority = 3)]
    // FloatNumber(f64),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#)] // |lex| lex.slice().to_owned())]
    String, //(String),

    // boolean ion single Boolean(bool)
    // #[regex(r"true|false", |lex| lex.slice() == "true")]
    // Boolean(bool),

    #[token("true")]
    True,

    #[token("false")]
    False,

    // Keywords.
    #[token("and")]
    AND,

    // #[token("class")]
    // CLASS,
    #[token("else")]
    ELSE,

    #[token("fn")]
    FN,

    #[token("for")]
    FOR,

    #[token("if")]
    IF,

    #[token("nil")]
    NIL,
    
    #[token("or")]
    OR,

    #[token("print")]
    PRINT,

    #[token("return")]
    RETURN,

    // #[token("super")]
    // SUPER,

    // #[token("this")]
    // THIS,
    #[token("let")]
    LET,

    #[token("while")]
    WHILE,

    // NOTE: Common Regex - https://github.com/maciejhirsz/logos/issues/133
    #[regex(r#"//[^\n]*"#)]
    COMMENT,

    EOF,
}


#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<ValueType>,
    pub span: std::ops::Range<usize>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let() {
        let mut lexer = TokenType::lexer("let x = 10;");
        assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("x".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::Number)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
    }

    #[test]
    fn test_string() {
        let mut lexer = TokenType::lexer("let a = \"Hello, let b World!\";");
        assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("a".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::String))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
    }

    #[test]
    fn test_comment() {
        let mut lexer = TokenType::lexer("let a = 10; // This is a comment");
        assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("a".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::Number)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::COMMENT)));
    }

    #[test]
    fn test_addequal() {
        let mut lexer = TokenType::lexer("let a = 4; a += 6; print(a == 10)");
        assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("a".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::Number)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("a".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::PLUS_EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::Number)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::PRINT)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::LEFT_PAREN)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("a".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL_EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::Number)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::RIGHT_PAREN)));
    }

    #[test]
    fn test_boolean() {
        let mut lexer = TokenType::lexer("let true_false_a = 4; let a = true; let b = false;");
        assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("true_false_a".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::Number)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("a".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::True)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
        assert_eq!(
            lexer.next(),
            Some(Ok(TokenType::Identifier("b".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::False)));
        assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
    }
}
