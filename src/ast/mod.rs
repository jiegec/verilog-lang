use crate::parser::Parser;
use serde::{Deserialize, Serialize};

pub mod attribute;
pub mod identifier;
pub mod module;

type TokenIndex = usize;

/// A.1.2 SystemVerilog source text
/// source_text ::= { description }
/// description ::= module_declaration
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct SourceText {
    pub modules: Vec<module::ModuleDeclaration>,
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Option<Self>;
}

impl Parse for SourceText {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = SourceText::default();
        while parser.avail() {
            if let Some(module) = module::ModuleDeclaration::parse(parser) {
                res.modules.push(module);
            }
        }
        Some(res)
    }
}
