use logos::Logos;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other,
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(error = LexingError)]
pub enum TokenType {
    // Single-character tokens.
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

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

    #[token("@")]
    AT, // dot product

    #[token("**")]
    StarStar, //  exponentiation

    // One or two character tokens.
    #[token("!")]
    BANG,

    #[token("!=")]
    BangEqual,

    #[token("=")]
    EQUAL,

    #[token("==")]
    EqualEqual,

    #[token(">")]
    GREATER,

    #[token(">=")]
    GreaterEqual,

    #[token("<")]
    LESS,

    #[token("<=")]
    LessEqual,

    #[token("+=")]
    PlusEqual, // +=

    #[token("-=")]
    MinusEqual, // -=

    #[token("*=")]
    StarEqual, // *=

    #[token("/=")]
    SlashEqual, // /=

    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_]*"#)]
    Identifier,

    // #[regex(r"(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    // Number(f64),

    #[regex(r"-?(?:0|[1-9]\d*)", |lex| lex.slice().parse::<i64>().unwrap())]
    IntNumber(i64),

    #[regex(r"-?(?:0|[1-9]\d*)\.\d+", |lex| lex.slice().parse::<f64>().unwrap())]
    FloatNumber(f64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#)]
    String,

    // boolean ion single Boolean(bool)
    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Boolean(bool),

    // Keywords.
    #[token("and")]
    AND,

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

    #[token("let")]
    LET,

    #[token("while")]
    WHILE,

    // NOTE: Common Regex - https://github.com/maciejhirsz/logos/issues/133
    #[regex(r#"//[^\n]*"#, logos::skip)]
    COMMENT,

    #[end]
    EOF, // end of file
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // pub literal: Option<ValueType>,
    pub span: std::ops::Range<usize>,
}

pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let mut lexer = TokenType::lexer(&source);
        let mut tokens = Vec::new();

        loop {
            let token = match lexer.next() {
                Some(Ok(token)) => token,
                Some(Err(e)) => {
                    println!("Error: {:?}", e);
                    break;
                }
                None => break,
            };

            tokens.push(Token {
                token_type: token,
                lexeme: lexer.slice().to_string(),
                // literal: value,
                span: lexer.span(),
            });
        }

        tokens.reverse();

        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            // literal: None,
            span: 0..0,
        })
    }

    pub fn peek(&self) -> Token {
        self.tokens
            .last()
            .clone()
            .unwrap_or(&Token {
                token_type: TokenType::EOF,
                lexeme: String::new(),
                // literal: None,
                span: 0..0,
            })
            .clone()
    }

    pub fn peek_n_type(&self, n: usize) -> Vec<TokenType> {
        let mut tokens = self.tokens.clone();
        tokens.reverse();
        tokens.truncate(n);
        // tokens.reverse();

        tokens.iter().map(|t| t.token_type).collect()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_let() {
//         let mut lexer = TokenType::lexer("let x = 10;");
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Number(10.))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//     }

//     #[test]
//     fn test_string() {
//         let mut lexer = TokenType::lexer("let a = \"Hello, let b World!\";");
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::String)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//     }

//     #[test]
//     fn test_comment() {
//         let mut lexer = TokenType::lexer("let a = 10; // This is a comment");
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Number(10.))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//     }

//     #[test]
//     fn test_addequal() {
//         let mut lexer = TokenType::lexer("let a = 4; a += 6; print(a == 10)");
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Number(4.))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::PlusEqual)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Number(6.))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::PRINT)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LeftParen)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EqualEqual)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Number(10.))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::RightParen)));
//     }

//     #[test]
//     fn test_boolean() {
//         let mut lexer = TokenType::lexer("let true_false_a = 4; let a = true; let b = false;");
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Number(4.))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Boolean(true))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::LET)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Identifier)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::EQUAL)));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::Boolean(false))));
//         assert_eq!(lexer.next(), Some(Ok(TokenType::SEMICOLON)));
//     }
// }
