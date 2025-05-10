pub mod ast;
pub mod lexer;
pub mod lower;

use std::iter::{self};

use ast::*;

pub use lexer::tokenize;
pub use lower::lower_stmt;

type ParseResult<T> = Result<T, String>;

struct Parser<'a, I: Iterator<Item = Token>> {
    tokens: &'a mut iter::Peekable<I>,
}

impl<I: iter::Iterator<Item = Token>> Parser<'_, I> {
    fn take(&mut self) -> ParseResult<Token> {
        let token = self
            .tokens
            .next()
            .ok_or("Unexpectedly reach end of file".to_owned())?;

        if let TokenKind::Error(msg) = token.kind {
            Err(msg.to_owned())
        } else {
            Ok(token)
        }
    }

    fn peek(&mut self) -> ParseResult<&Token> {
        return self
            .tokens
            .peek()
            .ok_or("Unexpectedly reach end of file".to_owned());
    }

    fn expect(&mut self, expected: TokenKind) -> ParseResult<Token> {
        match self.take()? {
            token if token.kind == expected => Ok(token),
            unexpected => Err(format!("Unexpectedly got '{}'", unexpected.value)),
        }
    }

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(TokenKind::Return)?;
        let return_val: Result<Expr, String> = self.parse_expr();
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(return_val?))
    }

    fn parse_unaryop(&mut self) -> ParseResult<UnaryOp> {
        match self.take()?.kind {
            TokenKind::Complement => Ok(UnaryOp::Complement),
            TokenKind::Negate => Ok(UnaryOp::Negate),
            _ => Err("".to_owned()),
        }
    }

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        let expr = match self.peek()?.kind {
            TokenKind::Constant => {
                let token = self.expect(TokenKind::Constant)?;
                Expr::Constant(token.value.parse().unwrap())
            }
            TokenKind::Negate | TokenKind::Complement => {
                let op = self.parse_unaryop()?;
                let inner_expr = self.parse_expr()?;
                Expr::Unary(op, Box::new(inner_expr))
            }
            TokenKind::LParen => {
                self.expect(TokenKind::LParen)?;
                let inner_expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                inner_expr
            }
            _ => return Err("Malformed expression".to_owned()),
        };

        Ok(expr)
    }

    fn parse_function(&mut self) -> ParseResult<Decl> {
        self.expect(TokenKind::Int)?;
        let name = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::LParen)?;
        self.expect(TokenKind::Void)?;
        self.expect(TokenKind::RParen)?;

        self.expect(TokenKind::LBrace)?;
        let body = self.parse_statement()?;
        self.expect(TokenKind::RBrace)?;

        Ok(Decl::Function(name.value, Box::new(body)))
    }

    fn parse_program(&mut self) -> ParseResult<Program> {
        let func = self.parse_function()?;

        Ok(Program { body: func })
    }
}

pub fn parse<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Program> {
    let mut tokens = tokens.peekable();
    let mut parser = Parser {
        tokens: &mut tokens,
    };
    let prg = parser.parse_program()?;

    if let Some(tok) = tokens.next() {
        return Err(format!(
            "Expected end of program, but got {:?} '{}'",
            tok.kind, tok.value
        ));
    }

    Ok(prg)
}
