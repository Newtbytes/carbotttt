use std::iter::Peekable;
use std::str;

use super::ast::{Token, TokenKind};

struct Scanner<'a> {
    src: Peekable<str::Chars<'a>>,
    consumed: String,
}

impl Scanner<'_> {
    fn eat(&mut self) -> Option<char> {
        let c = self.src.next();

        if let Some(c) = c {
            self.consumed.push(c);
        }

        c
    }

    fn eat_if<P>(&mut self, mut predicate: P) -> Option<char>
    where
        P: FnMut(&char) -> bool,
    {
        let ch = self.src.next_if(&mut predicate);

        if let Some(c) = ch {
            self.consumed.push(c);
        }

        ch
    }

    fn eat_while<P>(&mut self, mut predicate: P)
    where
        P: FnMut(&char) -> bool,
    {
        while self.eat_if(&mut predicate).is_some() {
            continue;
        }
    }

    fn one_ahead(&mut self) -> Option<&char> {
        self.src.peek()
    }

    fn emit(&mut self, token: TokenKind) -> Token {
        let tok = Token {
            kind: token,
            value: self.consumed.clone(),
        };

        self.consumed.clear();

        tok
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        use TokenKind::*;

        // skip whitespace
        self.eat_while(|&c| c.is_whitespace());
        self.consumed.clear();

        let kind = match self.eat() {
            Some(c) => match c {
                '(' => LParen,
                ')' => RParen,

                '{' => LBrace,
                '}' => RBrace,

                ';' => Semicolon,

                'a'..='z' | 'A'..='Z' | '_' => {
                    self.eat_while(|&c| matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_'));

                    // handle keywords
                    match self.consumed.as_str() {
                        "void" => Void,
                        "int" => Int,
                        "return" => Return,

                        _ => Identifier,
                    }
                }

                c if c.is_ascii_digit() => {
                    self.eat_while(|&c| c.is_ascii_digit());

                    if let Some(c) = self.one_ahead() {
                        if c.is_alphanumeric() {
                            panic!("Lexer error");
                        }
                    }

                    Constant
                }

                _ => panic!("lexer error"),
            },
            None => {
                return None;
            }
        };

        Some(self.emit(kind))
    }
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let src = src.chars().peekable();
    let scanner: Scanner = Scanner {
        src,
        consumed: String::new(),
    };

    scanner.collect()
}

pub fn tokens(src: &str) -> impl Iterator<Item = Token> {
    Scanner {
        src: src.chars().peekable(),
        consumed: String::new(),
    }
}
