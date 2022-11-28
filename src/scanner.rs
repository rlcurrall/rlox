use std::cell::RefCell;

use crate::{
    error::{Result, RuntimeError},
    token::{Token, TokenLiteral, TokenType},
};

#[derive(Debug)]
pub(crate) struct Scanner {
    chars: Vec<char>,
    current: RefCell<i32>,
    line: RefCell<i32>,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        Self {
            chars: source.clone().chars().into_iter().collect(),
            current: RefCell::new(0),
            line: RefCell::new(1),
        }
    }

    pub(crate) fn scan_tokens(self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        loop {
            if self.is_at_end(self.current()) {
                break;
            }

            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(TokenType::Eof, "".into(), None, self.line()));

        Ok(tokens)
    }

    fn scan_token(&self) -> Result<Option<Token>> {
        let next_char = self.advance();
        let next_pos = self.current();

        match next_char {
            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.line.replace(self.line() + 1);
                Ok(None)
            }
            '(' => Ok(Some(Token::new(
                TokenType::LeftParen,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            ')' => Ok(Some(Token::new(
                TokenType::RightParen,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            '{' => Ok(Some(Token::new(
                TokenType::LeftBrace,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            '}' => Ok(Some(Token::new(
                TokenType::RightBrace,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            ',' => Ok(Some(Token::new(
                TokenType::Comma,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            '.' => Ok(Some(Token::new(
                TokenType::Dot,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            '+' => Ok(Some(Token::new(
                TokenType::Plus,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            '-' => Ok(Some(Token::new(
                TokenType::Minus,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            ';' => Ok(Some(Token::new(
                TokenType::Semicolon,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            '*' => Ok(Some(Token::new(
                TokenType::Star,
                next_char.to_string(),
                None,
                self.line(),
            ))),
            '!' => {
                if !self.is_at_end(next_pos) && self.matches_at("=", next_pos) {
                    self.advance();
                    Ok(Some(Token::new(
                        TokenType::BangEqual,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenType::Bang,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                }
            }
            '=' => match !self.is_at_end(next_pos) && self.matches_at("=", next_pos) {
                true => {
                    self.advance();
                    Ok(Some(Token::new(
                        TokenType::EqualEqual,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                }
                false => Ok(Some(Token::new(
                    TokenType::Equal,
                    next_char.to_string(),
                    None,
                    self.line(),
                ))),
            },
            '>' => {
                if !self.is_at_end(next_pos) && self.matches_at("=", next_pos) {
                    self.advance();
                    Ok(Some(Token::new(
                        TokenType::GreaterEqual,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenType::Greater,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                }
            }
            '<' => {
                if !self.is_at_end(next_pos) && self.matches_at("=", next_pos) {
                    self.advance();
                    Ok(Some(Token::new(
                        TokenType::LessEqual,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenType::Less,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                }
            }
            '/' => {
                if !self.is_at_end(next_pos) && self.matches_at("/", next_pos) {
                    self.skip_inline_comment();
                    Ok(None)
                } else {
                    Ok(Some(Token::new(
                        TokenType::Slash,
                        next_char.to_string(),
                        None,
                        self.line(),
                    )))
                }
            }
            '"' => self.parse_string(),
            character => {
                if character.is_digit(10) {
                    self.parse_number()
                } else if character.is_alphabetic() {
                    self.parse_identifier()
                } else {
                    Err(RuntimeError::parse(
                        format!("Unexpected token: {character}"),
                        self.line(),
                        self.current(),
                    ))
                }
            }
        }
    }

    fn current(&self) -> i32 {
        self.current.clone().into_inner()
    }

    fn line(&self) -> i32 {
        self.line.clone().into_inner()
    }

    fn advance(&self) -> char {
        let current = self.current();
        self.current.replace(current + 1);
        self.chars[current as usize]
    }

    fn back(&self) -> char {
        let current = self.current();
        self.current.replace(current - 1);
        self.chars[current as usize]
    }

    fn peek(&self, offset: i32) -> Result<char> {
        match offset >= self.chars.len() as i32 {
            false => Ok(self.chars[offset as usize]),
            true => Err(RuntimeError::parse(
                format!("Attempt to read source at invalid offset `{offset}``"),
                self.line(),
                self.current(),
            )),
        }
    }

    fn matches_at(&self, expected: &str, offset: i32) -> bool {
        match self.peek(offset) {
            Ok(actual) => actual.to_string() == expected,
            Err(_) => false,
        }
    }

    fn is_at_end(&self, offset: i32) -> bool {
        self.chars.len() as i32 == offset
    }

    fn skip_inline_comment(&self) {
        if self.is_at_end(self.current()) {
            return;
        }

        loop {
            let next_char = self.advance();
            let next_pos = self.current();

            if next_char == '\n' || self.is_at_end(next_pos) {
                return;
            }
        }
    }

    fn parse_string(&self) -> Result<Option<Token>> {
        let mut lexeme = String::from("");

        loop {
            let next_char = self.advance();
            let current = self.current();

            if self.is_at_end(current) {
                return Err(RuntimeError::parse(
                    "Unterminated string".into(),
                    self.line(),
                    self.current(),
                ));
            }

            if next_char == '"' {
                break;
            }

            if next_char == '\n' {
                self.line.replace(self.line() + 1);
            }

            lexeme.push(next_char);
        }

        Ok(Some(Token::new(
            TokenType::String,
            lexeme.clone(),
            Some(TokenLiteral::String(lexeme)),
            self.line(),
        )))
    }

    fn parse_number(&self) -> Result<Option<Token>> {
        let mut lexeme = String::from("");
        self.back();

        loop {
            let next_char = self.advance();
            let next_pos = self.current();

            if self.is_at_end(next_pos) {
                break;
            }

            if !next_char.is_digit(10) {
                self.back();
                break;
            }

            lexeme.push(next_char);
        }

        let number = lexeme.parse::<i64>().map_err(|_| {
            RuntimeError::parse(
                format!("Could not parse number: `{lexeme}`"),
                self.line(),
                self.current(),
            )
        })?;

        return Ok(Some(Token::new(
            TokenType::Number,
            lexeme,
            Some(TokenLiteral::Number(number)),
            self.line(),
        )));
    }

    fn parse_identifier(&self) -> Result<Option<Token>> {
        let mut lexeme = String::from("");
        self.back();

        loop {
            let next_char = self.advance();
            let current = self.current();

            if self.is_at_end(current) {
                break;
            }

            if !next_char.is_alphabetic() {
                self.back();
                break;
            }

            lexeme.push(next_char);
        }

        match lexeme.to_lowercase().as_str() {
            "and" => Ok(Some(Token::new(TokenType::And, lexeme, None, self.line()))),
            "class" => Ok(Some(Token::new(
                TokenType::Class,
                lexeme,
                None,
                self.line(),
            ))),
            "else" => Ok(Some(Token::new(TokenType::Else, lexeme, None, self.line()))),
            "false" => Ok(Some(Token::new(
                TokenType::False,
                lexeme,
                None,
                self.line(),
            ))),
            "for" => Ok(Some(Token::new(TokenType::For, lexeme, None, self.line()))),
            "fun" => Ok(Some(Token::new(TokenType::Fun, lexeme, None, self.line()))),
            "if" => Ok(Some(Token::new(TokenType::If, lexeme, None, self.line()))),
            "nil" => Ok(Some(Token::new(TokenType::Nil, lexeme, None, self.line()))),
            "or" => Ok(Some(Token::new(TokenType::Or, lexeme, None, self.line()))),
            "print" => Ok(Some(Token::new(
                TokenType::Print,
                lexeme,
                None,
                self.line(),
            ))),
            "return" => Ok(Some(Token::new(
                TokenType::Return,
                lexeme,
                None,
                self.line(),
            ))),
            "super" => Ok(Some(Token::new(
                TokenType::Super,
                lexeme,
                None,
                self.line(),
            ))),
            "this" => Ok(Some(Token::new(TokenType::This, lexeme, None, self.line()))),
            "true" => Ok(Some(Token::new(TokenType::True, lexeme, None, self.line()))),
            "var" => Ok(Some(Token::new(TokenType::Var, lexeme, None, self.line()))),
            "while" => Ok(Some(Token::new(
                TokenType::While,
                lexeme,
                None,
                self.line(),
            ))),
            _ => Ok(Some(Token::new(
                TokenType::Identifier,
                lexeme,
                None,
                self.line(),
            ))),
        }
    }
}
