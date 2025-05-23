use crate::parser::ParseError;
use syntax::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(ParseError),
    Placeholder,
}
