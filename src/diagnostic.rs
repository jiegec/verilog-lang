use crate::lexer::Span;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Diagnostic {
    pub pos: Span,
    pub message: String,
}