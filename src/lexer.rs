use logos::Logos;

// pub struct Lexer {
//     src: String,
//     tokens: Vec<Token>,

//     start: usize,
//     current: usize,
//     line: u32,
// }

// impl Lexer {
//     pub fn new(src: &str) -> Self {
//         Self {
//             src: src.to_string(),
//             tokens: Vec::new(),

//             start: 0,
//             current: 0,
//             line: 1,
//         }
//     }

//     pub fn scan_tokens(&mut self) -> Vec<String> {
//         while !self.is_at_end() {
//             self.start = self.current;
//             self.scan_token();
//         }

//         self.tokens
//             .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

//         self.tokens.iter().map(|t| format!("{}", t)).collect()
//     }

//     fn scan_token(&mut self) {
//         let c = self.advance();
//         match c {
//             '(' => self.add_token(TokenType::LEFT_PAREN),
//             ')' => self.add_token(TokenType::RIGHT_PAREN),
//             '{' => self.add_token(TokenType::LEFT_BRACE),
//             '}' => self.add_token(TokenType::RIGHT_BRACE),
//             ',' => self.add_token(TokenType::COMMA),
//             '.' => self.add_token(TokenType::DOT),
//             '-' => self.add_token(TokenType::MINUS),
//             '+' => self.add_token(TokenType::PLUS),
//             '^' => self.add_token(TokenType::HAT),
//             ';' => self.add_token(TokenType::SEMICOLON),
//             '*' => self.add_token(TokenType::STAR),
//             '!' => {
//                 let token_type = if self.match_char('=') {
//                     TokenType::BANG_EQUAL
//                 } else {
//                     TokenType::BANG
//                 };
//                 self.add_token(token_type);
//             }
//             '=' => {
//                 let token_type = if self.match_char('=') {
//                     TokenType::EQUAL_EQUAL
//                 } else {
//                     TokenType::EQUAL
//                 };
//                 self.add_token(token_type);
//             }
//             '<' => {
//                 let token_type = if self.match_char('=') {
//                     TokenType::LESS_EQUAL
//                 } else {
//                     TokenType::LESS
//                 };
//                 self.add_token(token_type);
//             }
//             '>' => {
//                 let token_type = if self.match_char('=') {
//                     TokenType::GREATER_EQUAL
//                 } else {
//                     TokenType::GREATER
//                 };
//                 self.add_token(token_type);
//             }
//             '/' => {
//                 if self.match_char('/') {
//                     while self.peek() != '\n' && !self.is_at_end() {
//                         self.advance();
//                     }
//                 } else {
//                     self.add_token(TokenType::SLASH);
//                 }
//             }
//             ' ' | '\r' | '\t' => {}
//             '\n' => self.line += 1,
//             // '"' => self.string(),
//             _ => {
//                 // if c.is_digit(10) {
//                 //     self.number();
//                 // } else if c.is_alphabetic() {
//                 //     self.identifier();
//                 // } else {
//                 println!("Unexpected character: {}", c);
//                 // }
//             }
//         }
//     }

//     fn match_char(&mut self, expected: char) -> bool {
//         if self.is_at_end() {
//             return false;
//         }
//         if self.src.chars().nth(self.current).unwrap() != expected {
//             return false;
//         }

//         self.current += 1;
//         true
//     }

//     fn peek(&self) -> char {
//         if self.is_at_end() {
//             return '\0';
//         }
//         self.src.chars().nth(self.current).unwrap()
//     }

//     fn is_at_end(&self) -> bool {
//         self.current >= self.src.chars().count()
//     }

//     fn advance(&mut self) -> char {
//         let c = self.src.chars().nth(self.current).unwrap();
//         self.current += 1;
//         c
//     }

//     fn add_token(&mut self, token_type: TokenType) {
//         let text = self.src[self.start..self.current].to_string();
//         self.tokens
//             .push(Token::new(token_type, text, None, self.line));
//     }
// }

#[derive(Logos, Debug, PartialEq)]
// #[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
#[logos(skip r"[ \t\r\n\f]+")]
// #[logos(skip r"(?:[^\/\n]*\/\/[^\n]*\n)|[ \t\r\n\f]+")]
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

    // #[regex(r#""[^"]*""#)]
    // STRING,

    // #[regex(r#"[0-9]+"#)]
    // NUMBER,
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    String(String),

    // Keywords.
    #[token("and")]
    AND,

    // #[token("class")]
    // CLASS,
    #[token("else")]
    ELSE,

    #[token("false")]
    FALSE,

    #[token("fn")]
    FN,

    #[token("for")]
    FOR,

    #[token("if")]
    IF,

    // #[token("nil")]
    // NIL,
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
    #[token("true")]
    TRUE,

    #[token("let")]
    LET,

    #[token("while")]
    WHILE,


    // NOTE: Common Regex - https://github.com/maciejhirsz/logos/issues/133
    #[regex(r#"//[^\n]*"#)]
    COMMENT,

    EOF,
}

// pub struct Token {
//     token_type: TokenType,
//     lexeme: String,
//     literal: Option<String>,
//     line: u32,
// }

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "{:?} {} {}",
//             self.token_type,
//             self.lexeme,
//             match &self.literal {
//                 Some(l) => l,
//                 None => "",
//             }
//         )
//     }
// }

// impl Token {
//     pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: u32) -> Self {
//         Self {
//             token_type,
//             lexeme,
//             literal,
//             line,
//         }
//     }
// }
