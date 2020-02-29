use crate::lexer::Span;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    MultilineCommentUnclosed,
    UnrecognizedEscapeCharacter(char),
    UnexpectedChar(char),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use Message::*;
        match self {
            MultilineCommentUnclosed => write!(f, "Multiline Comment is not closed"),
            UnrecognizedEscapeCharacter(ch) => write!(f, "Unrecognized escape character: {}", ch),
            UnexpectedChar(ch) => write!(f, "Unexpected character: {}", ch),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Diagnostic {
    pub pos: Span,
    pub msg: Message,
    pub severity: Severity,
}
