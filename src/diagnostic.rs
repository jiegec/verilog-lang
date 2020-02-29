use crate::lexer::Span;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Diagnostic {
    pub pos: Span,
    pub message: String,
}
