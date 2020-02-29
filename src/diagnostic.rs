use serde::{Serialize, Deserialize};
use crate::lexer::Span;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Diagnostic {
    pub pos: Span,
    pub message: String,
}
