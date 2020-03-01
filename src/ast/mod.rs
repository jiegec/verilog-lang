use crate::{diagnostic::Message, lexer::Token, parser::Parser};
use serde::{Deserialize, Serialize};

pub mod attribute;
pub mod identifier;
pub mod module;

type TokenIndex = usize;

// A.1.3 Module and primitive source text
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct SourceText {
    pub modules: Vec<ModuleDeclaration>,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleDeclaration {
    pub attributes: Vec<attribute::Attribute>,
    pub identifier: identifier::Identifier,
}

trait Parse: Sized {
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
            let attr = attribute::Attribute::parse(parser)?;
            res.attributes.push(attr);
        }
        if parser.probe(&[Token::Module, Token::MacroModule]) {
            parser.advance();
            let identifier = identifier::Identifier::parse(parser)?;
            res.identifier = identifier;
            if parser.probe(&[Token::Sharp]) {
                // TODO: module_paramter_port_list
            }
            if parser.probe(&[Token::LParen]) {
                // TODO: list_of_ports
            }
            if parser.probe(&[Token::Semicolon]) {
                parser.advance();
                // TODO: module item
                if parser.probe(&[Token::EndModule]) {
                    parser.advance();
                    // TODO: module item
                    return Some(res);
                }
            }
        } else {
            parser.err(
                parser.location(),
                parser.location(),
                Message::UnexpectedTokens(
                    vec![Token::Module, Token::MacroModule],
                    "end of file".to_owned(),
                ),
            );
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module() {
        let mut parser = Parser::from("module test; endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        println!("{:?}", parser);
        assert!(m.is_some());
        assert_eq!(m.unwrap().identifier.token, 1);
    }
}
