#![deny(clippy::all)]

mod event;
mod grammar;
mod parser;
mod sink;
mod source;

use crate::parser::{ParseError, Parser};
use lexer::Lexer;
use rowan::GreenNode;
use sink::Sink;
use source::Source;
use syntax::SyntaxNode;

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);
    sink.finish()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let tree = format!("{:#?}", self.syntax());

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        let mut s = tree.trim_end().to_string();

        s.extend(self.errors.iter().map(|error| format!("\n{error}")));

        s
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}
