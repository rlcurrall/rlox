use crate::{
    error::{Result, RuntimeError},
    token::{Token, TokenValue},
};

#[derive(Debug)]
pub(crate) struct Scanner {
    chars: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        let chars = source.clone().chars().into_iter().collect();

        Self {
            chars,
            position: 0,
            line: 1,
            column: 0,
        }
    }

    pub(crate) fn scan_tokens(mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        if self.chars.is_empty() {
            tokens.push(Token::new(TokenValue::Eof, "".into(), self.line));
            return Ok(tokens);
        }

        loop {
            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }

            if self.at_end() {
                break;
            }

            self.advance();
        }

        tokens.push(Token::new(TokenValue::Eof, "".into(), self.line));

        Ok(tokens)
    }

    fn at_end(&self) -> bool {
        self.peek().is_err()
    }

    fn current(&self) -> char {
        self.chars[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
        self.column += 1;
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    fn peek(&self) -> Result<char> {
        let offset = self.position + 1;
        match offset >= self.chars.len() {
            false => Ok(self.chars[offset]),
            true => Err(RuntimeError::scan_error(
                format!("Attempt to read source at invalid offset `{offset}``"),
                self.line,
                self.position,
            )),
        }
    }

    fn next_eq(&self, expected: &str) -> bool {
        match self.peek() {
            Ok(actual) => actual.to_string() == expected,
            Err(_) => false,
        }
    }

    fn scan_token(&mut self) -> Result<Option<Token>> {
        let next_char = self.current();
        let lexeme = next_char.to_string();

        match next_char {
            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.next_line();
                Ok(None)
            }
            '(' => Ok(Some(Token::new(TokenValue::LeftParen, lexeme, self.line))),
            ')' => Ok(Some(Token::new(TokenValue::RightParen, lexeme, self.line))),
            '{' => Ok(Some(Token::new(TokenValue::LeftBrace, lexeme, self.line))),
            '}' => Ok(Some(Token::new(TokenValue::RightBrace, lexeme, self.line))),
            ',' => Ok(Some(Token::new(TokenValue::Comma, lexeme, self.line))),
            '.' => Ok(Some(Token::new(TokenValue::Dot, lexeme, self.line))),
            '+' => Ok(Some(Token::new(TokenValue::Plus, lexeme, self.line))),
            '-' => Ok(Some(Token::new(TokenValue::Minus, lexeme, self.line))),
            ';' => Ok(Some(Token::new(TokenValue::Semicolon, lexeme, self.line))),
            '*' => Ok(Some(Token::new(TokenValue::Star, lexeme, self.line))),
            '!' => match self.next_eq("=") {
                false => Ok(Some(Token::new(TokenValue::Bang, lexeme, self.line))),
                true => {
                    self.advance();
                    Ok(Some(Token::new(TokenValue::BangEqual, lexeme, self.line)))
                }
            },
            '=' => match self.next_eq("=") {
                false => Ok(Some(Token::new(TokenValue::Equal, lexeme, self.line))),
                true => {
                    self.advance();
                    Ok(Some(Token::new(TokenValue::EqualEqual, lexeme, self.line)))
                }
            },
            '>' => match self.next_eq("=") {
                false => Ok(Some(Token::new(TokenValue::Greater, lexeme, self.line))),
                true => {
                    self.advance();
                    Ok(Some(Token::new(
                        TokenValue::GreaterEqual,
                        lexeme,
                        self.line,
                    )))
                }
            },
            '<' => match self.next_eq("=") {
                false => Ok(Some(Token::new(TokenValue::Less, lexeme, self.line))),
                true => {
                    self.advance();
                    Ok(Some(Token::new(TokenValue::LessEqual, lexeme, self.line)))
                }
            },
            '/' => match self.next_eq("/") {
                false => Ok(Some(Token::new(TokenValue::Slash, lexeme, self.line))),
                true => {
                    self.skip_inline_comment();
                    Ok(None)
                }
            },
            '"' => self.scan_string(),
            character => {
                if character.is_digit(10) {
                    self.scan_number()
                } else if character.is_alphabetic() || character == '_' {
                    self.scan_identifier()
                } else {
                    Err(RuntimeError::scan_error(
                        format!("Unexpected token: {character}"),
                        self.line,
                        self.position,
                    ))
                }
            }
        }
    }

    fn skip_inline_comment(&mut self) {
        if self.at_end() {
            return;
        }

        loop {
            self.advance();
            if self.current() == '\n' || self.at_end() {
                return;
            }
        }
    }

    fn scan_string(&mut self) -> Result<Option<Token>> {
        let mut lexeme = String::from("");
        self.advance();

        loop {
            if self.at_end() {
                return Err(RuntimeError::scan_error(
                    "Unterminated string".into(),
                    self.line,
                    self.position,
                ));
            }

            let char = self.current();

            if char == '"' {
                break;
            }

            if char == '\n' {
                self.next_line();
            }

            lexeme.push(char);
            self.advance();
        }

        Ok(Some(Token::new(
            TokenValue::String(lexeme.clone()),
            lexeme,
            self.line,
        )))
    }

    fn scan_number(&mut self) -> Result<Option<Token>> {
        let mut lexeme = String::from("");
        let current = self.current();
        lexeme.push(current);

        loop {
            if self.at_end() {
                break;
            }

            if let Ok(char) = self.peek() {
                if !char.is_digit(10) {
                    break;
                }

                lexeme.push(char);
                self.advance();
            }
        }

        let number = lexeme.parse::<f64>().map_err(|_| {
            RuntimeError::scan_error(
                format!("Could not parse number: `{lexeme}`"),
                self.line,
                self.position,
            )
        })?;

        return Ok(Some(Token::new(
            TokenValue::Number(number),
            lexeme,
            self.line,
        )));
    }

    fn scan_identifier(&mut self) -> Result<Option<Token>> {
        let mut lexeme = String::from("");
        let char = self.current();
        lexeme.push(char);

        loop {
            if self.at_end() {
                break;
            }

            if let Ok(char) = self.peek() {
                if !char.is_alphabetic() && char != '_' {
                    break;
                }

                lexeme.push(char);
                self.advance();
            }
        }

        match lexeme.to_lowercase().as_str() {
            "and" => Ok(Some(Token::new(TokenValue::And, lexeme, self.line))),
            "class" => Ok(Some(Token::new(TokenValue::Class, lexeme, self.line))),
            "else" => Ok(Some(Token::new(TokenValue::Else, lexeme, self.line))),
            "false" => Ok(Some(Token::new(TokenValue::False, lexeme, self.line))),
            "for" => Ok(Some(Token::new(TokenValue::For, lexeme, self.line))),
            "fun" => Ok(Some(Token::new(TokenValue::Fun, lexeme, self.line))),
            "if" => Ok(Some(Token::new(TokenValue::If, lexeme, self.line))),
            "nil" => Ok(Some(Token::new(TokenValue::Nil, lexeme, self.line))),
            "or" => Ok(Some(Token::new(TokenValue::Or, lexeme, self.line))),
            "print" => Ok(Some(Token::new(TokenValue::Print, lexeme, self.line))),
            "return" => Ok(Some(Token::new(TokenValue::Return, lexeme, self.line))),
            "super" => Ok(Some(Token::new(TokenValue::Super, lexeme, self.line))),
            "this" => Ok(Some(Token::new(TokenValue::This, lexeme, self.line))),
            "true" => Ok(Some(Token::new(TokenValue::True, lexeme, self.line))),
            "var" => Ok(Some(Token::new(TokenValue::Var, lexeme, self.line))),
            "while" => Ok(Some(Token::new(TokenValue::While, lexeme, self.line))),
            _ => Ok(Some(Token::new(
                TokenValue::Identifier(lexeme.clone()),
                lexeme,
                self.line,
            ))),
        }
    }
}
