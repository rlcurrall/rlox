use crate::{
    error::{Result, RuntimeError},
    token::{Token, TokenValue},
};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    errors: Vec<RuntimeError>,
    position: usize,
}

/// Syntax Grammar for Lox
/// ======================
///
/// ## Declarations
/// A program is a series of declarations, which are the statements that bind
/// new identifiers or any of the other statement types.
///
/// ```
/// declaration → classDecl
///             | funDecl
///             | varDecl
///             | statement ;
///
/// classDecl   → "class" IDENTIFIER ( "<" IDENTIFIER )?
///               "{" function* "}" ;
/// funDecl     → "fun" function ;
/// varDecl     → "var" IDENTIFIER ( "=" expression )? ";" ;
/// ```
///
/// ## Statements
/// The remaining statement rules produce side effects, but do not introduce bindings.
///
/// Note that `block` is a statement rule, but is also used as a non-terminal in
/// a couple of other rules for things like function bodies.
///
/// ```
/// statement   → exprStmt
///             | forStmt
///             | ifStmt
///             | printStmt
///             | returnStmt
///             | whileStmt
///             | block ;
///
/// exprStmt    → expression ";" ;
/// forStmt     → "for" "(" ( varDecl | exprStmt | ";" )
///                         expression? ";"
///                         expression? ")" statement ;
/// ifStmt      → "if" "(" expression ")" statement
///               ( "else" statement )? ;
/// printStmt   → "print" expression ";" ;
/// returnStmt  → "return" expression? ";" ;
/// whileStmt   → "while" "(" expression ")" statement ;
/// block       → "{" declaration* "}" ;
/// ```
///
/// ## Expressions
/// Expressions produce values. Lox has a number of unary and binary operators
/// with different levels of precedence. Some grammars for languages do not
/// directly encode the precedence relationships and specify that elsewhere.
/// Here, we use a separate rule for each precedence level to make it explicit.
///
/// ```
/// expression  → assignment ;
///
/// assignment  → ( call "." )? IDENTIFIER "=" assignment
///             | logic_or ;
///
/// logic_or    → logic_and ( "or" logic_and )* ;
/// logic_and   → equality ( "and" equality )* ;
/// equality    → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term        → factor ( ( "-" | "+" ) factor )* ;
/// factor      → unary ( ( "/" | "*" ) unary )* ;
///
/// unary       → ( "!" | "-" ) unary | call ;
/// call        → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
/// primary     → "true" | "false" | "nil" | "this"
///             | NUMBER | STRING | IDENTIFIER | "(" expression ")"
///             | "super" "." IDENTIFIER ;
/// ```
///
/// ## Utility Rules
/// In order to keep the above rules a little cleaner, some of the grammar is
/// split out into a few reused helper rules.
///
/// ```
/// function    → IDENTIFIER "(" parameters? ")" block ;
/// parameters  → IDENTIFIER ( "," IDENTIFIER )* ;
/// arguments   → expression ( "," expression )* ;
/// ```
///
/// Lexical Grammar of Lox
/// ======================
/// The lexical grammar is used by the scanner to group characters into tokens. Where the syntax is context free, the
/// lexical grammar is regular—note that there are no recursive rules.
///
/// ```
/// NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
/// STRING      → "\"" <any char except "\"">* "\"" ;
/// IDENTIFIER  → ALPHA ( ALPHA | DIGIT )* ;
/// ALPHA       → "a" ... "z" | "A" ... "Z" | "_";
/// DIGIT       → "0" ... "9" ;
/// ```
///
impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            errors: vec![],
            position: 0,
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        let mut errors = vec![];

        if self.tokens.is_empty() {
            return Ok(statements);
        }

        loop {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => errors.push(err),
            };

            if self.is_at_end() {
                break;
            }

            self.advance();
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(RuntimeError::general_error("Errors occurred"))
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.position - 1].clone()
    }

    fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.position += 1;
        }

        self.current()
    }

    fn is_match(&mut self, types: &[TokenValue]) -> bool {
        match self.peek() {
            Err(_) => false,
            Ok(token) => types.contains(&token.value),
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_err()
    }

    fn peek(&self) -> Result<Token> {
        let offset = self.position + 1;
        match offset >= self.tokens.len() {
            false => Ok(self.tokens[offset].clone()),
            true => Err(RuntimeError::general_error(
                "Unexpected end of token stream",
            )),
        }
    }

    fn consume(&mut self, expected: TokenValue, message: &str) -> Result<Token> {
        let token = self.peek()?;

        if token.value == expected {
            self.advance();
            Ok(token)
        } else {
            Err(RuntimeError::ParseError(message.into(), token))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<Token> {
        let token = self.peek()?;

        match token.value {
            TokenValue::Identifier(_) => {
                self.advance();
                Ok(token)
            }
            _ => Err(RuntimeError::ParseError(message.into(), token)),
        }
    }

    fn declaration(&mut self) -> Result<Stmt> {
        self.advance();
        let token = self.current();

        match token.value {
            TokenValue::Class => todo!(),
            TokenValue::Fun => todo!(),
            TokenValue::Var => todo!(),
            _ => self.statement(),
        }
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.current().value {
            TokenValue::For => todo!(),
            TokenValue::If => todo!(),
            TokenValue::Print => todo!(),
            TokenValue::Return => todo!(),
            TokenValue::While => todo!(),
            TokenValue::LeftBrace => self.block(),
            _ => self.expression_statement(),
        }
    }

    // fn while_s

    fn block(&mut self) -> Result<Stmt> {
        let mut statements = vec![];

        while !self.is_match(&[TokenValue::RightBrace]) {
            statements.push(self.declaration()?);
        }

        self.consume(TokenValue::RightBrace, "Expected `}` after block")?;

        Ok(Stmt::Block(statements))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenValue::Semicolon, "Expected `;` after expression")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let mut expr = self.logic_or()?;

        if self.is_match(&[TokenValue::Equal]) {
            let value = self.assignment()?;
            expr = match expr {
                Expr::Variable { name } => Expr::Assign {
                    name,
                    value: Box::new(value),
                },
                Expr::Get { name, object } => Expr::Set {
                    object,
                    name,
                    value: Box::new(value),
                },
                _ => return Err(RuntimeError::InvalidArgumentTarget("todo".into())),
            };
        }

        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Expr> {
        let mut expr = self.logic_and()?;

        while self.is_match(&[TokenValue::Or]) {
            self.advance();
            let operator = self.current();
            let and = self.logic_and()?;
            expr = Expr::Logical {
                right: Box::new(expr),
                operator,
                left: Box::new(and),
            };
        }

        Ok(expr)
    }

    fn logic_and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenValue::And]) {
            self.advance();
            let operator = self.current();
            let equality = self.equality()?;
            expr = Expr::Logical {
                right: Box::new(expr),
                operator,
                left: Box::new(equality),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenValue::BangEqual, TokenValue::EqualEqual]) {
            self.advance();
            let operator = self.current();
            let factor = self.comparison()?;
            expr = Expr::Equality {
                operator,
                right: Box::new(factor),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.is_match(&[
            TokenValue::Greater,
            TokenValue::GreaterEqual,
            TokenValue::Less,
            TokenValue::LessEqual,
        ]) {
            self.advance();
            let operator = self.current();
            let factor = self.term()?;
            expr = Expr::Comparison {
                operator,
                right: Box::new(factor),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenValue::Minus, TokenValue::Plus]) {
            self.advance();
            let operator = self.current();
            let factor = self.factor()?;
            expr = Expr::Term {
                operator,
                right: Box::new(factor),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenValue::Star, TokenValue::Slash]) {
            self.advance();
            let operator = self.current();
            let unary = self.unary()?;
            expr = Expr::Factor {
                operator,
                right: Box::new(unary),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.is_match(&[TokenValue::Bang, TokenValue::Minus]) {
            self.advance();
            let operator = self.current();
            let unary = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(unary),
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.is_match(&[TokenValue::LeftParen]) {
                self.advance();
                let mut arguments = vec![];

                loop {
                    if arguments.len() > 255 {
                        // todo - improve this error
                        return Err(RuntimeError::general_error("Too many arguments"));
                    }

                    arguments.push(self.assignment()?);
                }

                todo!();
            } else if self.is_match(&[TokenValue::Dot]) {
                self.consume(TokenValue::Dot, "Expected `.`")?;
                let name = self.consume_identifier("Expected property name after `.`")?;
                expr = Expr::Get {
                    name,
                    object: Box::new(expr),
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.current();
        let res = match token.value.clone() {
            TokenValue::True => Expr::Literal(Literal::True),
            TokenValue::False => Expr::Literal(Literal::False),
            TokenValue::Nil => Expr::Literal(Literal::Nil),
            TokenValue::This => Expr::This { keyword: token },
            TokenValue::Number(n) => Expr::Literal(Literal::Number(n)),
            TokenValue::String(s) => Expr::Literal(Literal::String(s)),
            TokenValue::Identifier(_) => Expr::Variable { name: token },
            TokenValue::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenValue::RightParen, "Expected `)` after expression")?;
                Expr::Grouping {
                    group: Box::new(expr),
                }
            }
            TokenValue::Super => {
                self.consume(TokenValue::Dot, "Expected `.` after `super`")?;
                let method = self.consume_identifier("Expected superclass method name")?;
                Expr::Super {
                    keyword: token,
                    method,
                }
            }
            t => {
                return Err(RuntimeError::ParseError(
                    format!("Expected expression, found: `{t}`"),
                    token,
                ))
            }
        };

        Ok(res)
    }
}

pub enum Stmt {
    Expression(Expr),
    Block(Vec<Stmt>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    This {
        keyword: Token,
    },
    Variable {
        name: Token,
    },
    Grouping {
        group: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    Get {
        name: Token,
        object: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Factor {
        operator: Token,
        right: Box<Expr>,
    },
    Term {
        operator: Token,
        right: Box<Expr>,
    },
    Comparison {
        operator: Token,
        right: Box<Expr>,
    },
    Equality {
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
        right: Box<Expr>,
        operator: Token,
        left: Box<Expr>,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
pub enum Literal {
    False,
    True,
    Nil,
    Number(f64),
    String(String),
}
