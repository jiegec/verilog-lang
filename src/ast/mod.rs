use crate::{diagnostic::Message, lexer::Token, parser::Parser};
use serde::{Deserialize, Serialize};

mod declarations;
mod expressions;
mod general;
mod source_text;

pub use declarations::*;
pub use expressions::*;
pub use general::*;
pub use source_text::*;

type TokenIndex = usize;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Option<Self>;
}
