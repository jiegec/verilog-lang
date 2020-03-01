use serde::{Deserialize, Serialize};
use crate::{lexer::Token, parser::Parser};

pub mod module;
pub mod identifier;
pub mod attribute;

type TokenIndex = usize;

// A.1.3 Module and primitive source text
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct SourceText {
    pub modules: Vec<ModuleDeclaration>,
}

#[derive(PartialEq, Eq,  Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleDeclaration {
    pub attributes: Vec<attribute::Attribute>,
    pub identifier: identifier::Identifier,
}

trait Parse : Sized {
    fn parse(parser: &mut Parser) -> Option<Self>;
}

impl Parse for SourceText {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = SourceText::default();
        while parser.avail() {
            if let Some(module) = ModuleDeclaration::parse(parser) {
                res.modules.push(module);
            }
        }
        Some(res)
    }
}

impl Parse for ModuleDeclaration {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = ModuleDeclaration::default();
        while parser.probe(&[Token::LParen]) {
            if let Some(attr) = attribute::Attribute::parse(parser) {
                res.attributes.push(attr);
            }
        }
        if parser.probe(&[Token::Module, Token::MacroModule]) {
            if let Some(identifier) = identifier::Identifier::parse(parser) {
                res.identifier = identifier;
            }
        }
        Some(res)
    }
}