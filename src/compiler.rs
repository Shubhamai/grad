use logos::{Lexer, Logos};

use crate::{
    chunk::{Chunk, OpCode},
    debug::Disassemble,
    interner::Interner,
    scanner::{self, LexingError, TokenType},
    value::ValueType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<ValueType>,
    pub span: std::ops::Range<usize>,
}

#[derive(Debug)]
struct Parser {
    current: Token,
    previous: Token,
}

#[derive(Debug, Clone, Copy)]
enum Precedence {
    PREC_NONE,
    PREC_ASSIGNMENT, // =
    PREC_OR,         // or
    PREC_AND,        // and
    PREC_EQUALITY,   // == !=
    PREC_COMPARISON, // < > <= >=
    PREC_TERM,       // + -
    PREC_FACTOR,     // * /
    PREC_UNARY,      // ! -
    PREC_CALL,       // . ()
    PREC_PRIMARY,
}

pub type ParseFn = fn(&mut Compiler) -> ();

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

impl ParseRule {
    fn get_rule(token_type: TokenType) -> ParseRule {
        match token_type {
            TokenType::LEFT_PAREN => ParseRule {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::RIGHT_PAREN => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::LEFT_BRACE => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::RIGHT_BRACE => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::COMMA => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::DOT => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::MINUS => ParseRule {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_TERM,
            },
            TokenType::PLUS => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_TERM,
            },
            TokenType::HAT => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::SEMICOLON => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::SLASH => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_FACTOR,
            },
            TokenType::STAR => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_FACTOR,
            },
            TokenType::BANG => ParseRule {
                prefix: Some(Compiler::unary),
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::BANG_EQUAL => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_EQUALITY,
            },
            TokenType::EQUAL => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::EQUAL_EQUAL => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_EQUALITY,
            },
            TokenType::GREATER => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_COMPARISON,
            },
            TokenType::GREATER_EQUAL => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_COMPARISON,
            },
            TokenType::LESS => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_COMPARISON,
            },
            TokenType::LESS_EQUAL => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PREC_COMPARISON,
            },
            TokenType::PLUS_EQUAL => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_ASSIGNMENT,
            },
            TokenType::MINUS_EQUAL => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_ASSIGNMENT,
            },
            TokenType::STAR_EQUAL => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_ASSIGNMENT,
            },
            TokenType::SLASH_EQUAL => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_ASSIGNMENT,
            },
            TokenType::Identifier => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::Number => ParseRule {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::String => ParseRule {
                prefix: Some(Compiler::string),
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::True => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::False => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::AND => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_AND,
            },
            TokenType::ELSE => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::FN => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::FOR => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::IF => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::NIL => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::OR => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_OR,
            },
            TokenType::PRINT => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::RETURN => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::LET => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::WHILE => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::COMMENT => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
            TokenType::EOF => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PREC_NONE,
            },
        }
    }
}

pub(crate) struct Compiler {
    chunk: Chunk,
    pub tokens_vec: Vec<Token>,
    // pub tokens : Vec<(Result<TokenType, LexingError>, std::ops::Range<usize>)>,
    current_token: usize,

    parser: Parser,
    // interner: &'src mut Interner,
}

impl Compiler {
    pub fn new(source: String, interner: &mut Interner) -> Compiler {
        let mut tokens = TokenType::lexer(&source);
        // remove the lifetime from the tokens without use_ref
        // let tokens = TokenType::lexer(&source).spanned().collect();

        // convet tokens to Vec<TokenType>

        let mut tokens_vec = Vec::new();
        loop {
            let token = match tokens.next() {
                Some(Ok(token)) => token,
                Some(Err(e)) => {
                    println!("Error: {:?}", e);
                    break;
                }
                None => break,
            };

            let value = match token {
                TokenType::Number => {
                    Some(ValueType::Number(tokens.slice().parse::<f32>().unwrap()))
                }
                TokenType::String => Some(ValueType::String(interner.intern(tokens.slice()))),
                TokenType::True => Some(ValueType::Boolean(true)),
                TokenType::False => Some(ValueType::Boolean(false)),
                TokenType::NIL => Some(ValueType::Nil),
                _ => None,
            };

            tokens_vec.push(Token {
                token_type: token,
                lexeme: tokens.slice().to_string(),
                literal: value,
                span: tokens.span(),
            });
        }

        println!("{:?}", tokens_vec);

        Compiler {
            chunk: Chunk::new(),
            tokens_vec,
            // tokens,
            current_token: 0,
            parser: Parser {
                current: Token {
                    token_type: TokenType::EOF,
                    lexeme: "".to_string(),
                    literal: None,
                    span: 0..0,
                },
                previous: Token {
                    token_type: TokenType::EOF,
                    lexeme: "".to_string(),
                    literal: None,
                    span: 0..0,
                },
            },
            // interner,
        }
    }

    pub(crate) fn compile(&mut self) -> Result<Chunk, ()> {
        // TODO : 17.2.1 - Handling syntax errors

        self.advance();
        self.expression();
        self.end_compiler();
        Ok(self.chunk.clone())
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            if self.current_token >= self.tokens_vec.len() {
                self.parser.current = Token {
                    token_type: TokenType::EOF,
                    lexeme: "".to_string(),
                    literal: None,
                    span: 0..0,
                };
                return;
            }

            let token = self.tokens_vec[self.current_token].clone();
            self.parser.current = token.clone();

            self.current_token += 1;

            if token.token_type != TokenType::COMMENT {
                break;
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        // self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: usize) {
        self.chunk.write(byte, self.parser.previous.span.start);
    }

    fn emit_bytes(&mut self, byte1: usize, byte2: usize) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler(&mut self) {
        self.emit_byte(usize::from(OpCode::OpReturn));

        // debug
        self.chunk.disassemble("code");
    }

    fn number(&mut self) {
        let value = match self.parser.previous.literal.clone() {
            Some(value) => value,
            _ => panic!("Error: Expected a number."),
        };

        self.emit_constant(value);
    }

    fn string(&mut self) {
        let value = match self.parser.previous.literal.clone() {
            Some(value) => value,
            _ => panic!("Error: Expected a string."),
        };

        self.emit_constant(value);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PREC_ASSIGNMENT);
    }

    fn emit_constant(&mut self, value: ValueType) {
        let constant = self.chunk.add_constant(value);
        self.emit_bytes(usize::from(OpCode::OpConstant), constant);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        self.parse_precedence(Precedence::PREC_UNARY);

        match operator_type {
            TokenType::BANG => self.emit_byte(usize::from(OpCode::OpNot)),
            TokenType::MINUS => self.emit_byte(usize::from(OpCode::OpNegate)),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = ParseRule::get_rule(self.parser.previous.token_type).prefix;

        if prefix_rule.is_none() {
            // self.error("Expect expression.");
            println!("Expect expression.");
            return;
        }

        match prefix_rule {
            Some(rule) => rule(self),
            None => {}
        }

        while precedence as i32
            <= ParseRule::get_rule(self.parser.current.token_type).precedence as i32
        {
            self.advance();
            let infix_rule = ParseRule::get_rule(self.parser.previous.token_type).infix;

            match infix_rule {
                Some(rule) => rule(self),
                None => {}
            }
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        let rule = ParseRule::get_rule(operator_type);

        self.parse_precedence(rule.precedence as Precedence);

        match operator_type {
            TokenType::PLUS => self.emit_byte(usize::from(OpCode::OpAdd)),
            TokenType::MINUS => self.emit_byte(usize::from(OpCode::OpSubtract)),
            TokenType::STAR => self.emit_byte(usize::from(OpCode::OpMultiply)),
            TokenType::SLASH => self.emit_byte(usize::from(OpCode::OpDivide)),
            TokenType::BANG_EQUAL => {
                self.emit_bytes(usize::from(OpCode::OpEqual), usize::from(OpCode::OpNot))
            }
            TokenType::EQUAL_EQUAL => self.emit_byte(usize::from(OpCode::OpEqual)),
            TokenType::GREATER => self.emit_byte(usize::from(OpCode::OpGreater)),
            TokenType::GREATER_EQUAL => {
                self.emit_bytes(usize::from(OpCode::OpLess), usize::from(OpCode::OpNot))
            }
            TokenType::LESS => self.emit_byte(usize::from(OpCode::OpLess)),
            TokenType::LESS_EQUAL => {
                self.emit_bytes(usize::from(OpCode::OpGreater), usize::from(OpCode::OpNot))
            }
            _ => {}
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.token_type {
            TokenType::True => self.emit_byte(usize::from(OpCode::OpTrue)),
            TokenType::False => self.emit_byte(usize::from(OpCode::OpFalse)),
            TokenType::NIL => self.emit_byte(usize::from(OpCode::OpNil)),
            _ => {
                panic!("Error: Expected a literal.");
            }
        }
    }
}
