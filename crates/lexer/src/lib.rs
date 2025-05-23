#![deny(clippy::all)]

mod token_kind;
pub use token_kind::TokenKind;

use logos::Logos;
use std::ops::Range as StdRange;
use text_size::{TextRange, TextSize};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?.unwrap_or(TokenKind::Error);
        let text = self.inner.slice();
        let range = {
            let StdRange { start, end } = self.inner.span();
            let start = TextSize::try_from(start).unwrap();
            let end = TextSize::try_from(end).unwrap();
            TextRange::new(start, end)
        };

        Some(Self::Item { kind, text, range })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub range: TextRange,
}
