use std::iter;

use super::ast::*;

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
        self.tokens
            .peek()
            .ok_or("Unexpectedly reach end of file".to_owned())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Token, TokenKind};

    fn make_token(kind: TokenKind, value: &str) -> Token {
        Token { kind, value: value.to_string(), offset: 0 }
    }

    #[test]
    fn test_parse_constant_expr() {
        let tokens = vec![
            make_token(TokenKind::Constant, "42"),
        ];
        let mut iter = tokens.into_iter();
        let expr = Parser { tokens: &mut iter.peekable() }.parse_expr().unwrap();
        match expr {
            Expr::Constant(val) => assert_eq!(val, 42),
            _ => panic!("Expected constant expr"),
        }
    }

    #[test]
    fn test_parse_unary_expr() {
        let tokens = vec![
            make_token(TokenKind::Negate, "-"),
            make_token(TokenKind::Constant, "5"),
        ];
        let mut iter = tokens.into_iter();
        let expr = Parser { tokens: &mut iter.peekable() }.parse_expr().unwrap();
        match expr {
            Expr::Unary(UnaryOp::Negate, inner) => match *inner {
                Expr::Constant(val) => assert_eq!(val, 5),
                _ => panic!("Expected constant inside unary"),
            },
            _ => panic!("Expected unary expr"),
        }
    }

    #[test]
    fn test_parse_paren_expr() {
        let tokens = vec![
            make_token(TokenKind::LParen, "("),
            make_token(TokenKind::Constant, "7"),
            make_token(TokenKind::RParen, ")"),
        ];
        let mut iter = tokens.into_iter();
        let expr = Parser { tokens: &mut iter.peekable() }.parse_expr().unwrap();
        match expr {
            Expr::Constant(val) => assert_eq!(val, 7),
            _ => panic!("Expected constant expr in parens"),
        }
    }

    #[test]
    fn test_parse_statement() {
        let tokens = vec![
            make_token(TokenKind::Return, "return"),
            make_token(TokenKind::Constant, "1"),
            make_token(TokenKind::Semicolon, ";"),
        ];
        let mut iter = tokens.into_iter();
        let stmt = Parser { tokens: &mut iter.peekable() }.parse_statement().unwrap();
        match stmt {
            Stmt::Return(Expr::Constant(val)) => assert_eq!(val, 1),
            _ => panic!("Expected return statement with constant"),
        }
    }

    #[test]
    fn test_parse_function() {
        let tokens = vec![
            make_token(TokenKind::Int, "int"),
            make_token(TokenKind::Identifier, "main"),
            make_token(TokenKind::LParen, "("),
            make_token(TokenKind::Void, "void"),
            make_token(TokenKind::RParen, ")"),
            make_token(TokenKind::LBrace, "{"),
            make_token(TokenKind::Return, "return"),
            make_token(TokenKind::Constant, "0"),
            make_token(TokenKind::Semicolon, ";"),
            make_token(TokenKind::RBrace, "}"),
        ];
        let mut iter = tokens.into_iter();
        let decl = Parser { tokens: &mut iter.peekable() }.parse_function().unwrap();
        match decl {
            Decl::Function(name, body) => {
                assert_eq!(name, "main");
                match *body {
                    Stmt::Return(Expr::Constant(val)) => assert_eq!(val, 0),
                    _ => panic!("Expected return statement in function body"),
                }
            },
        }
    }
}
