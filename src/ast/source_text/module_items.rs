//! A.1.4 Module items

use crate::ast::*;

/// module_item ::= port_declaration ; | non_port_module_item
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum ModuleItem {
    Port(PortDeclaration),
    NonPort(NonPortModuleItem),
}

impl Parse for ModuleItem {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if parser.probe(&[
            Token::LParen,
            Token::InOut,
            Token::Input,
            Token::Output,
            Token::Ref,
            Token::Comma,
        ]) {
            if let Some(port) = PortDeclaration::parse(parser) {
                if parser.probe_err(&[Token::Comma]) {
                    parser.advance();
                    return Some(ModuleItem::Port(port));
                }
            }
        }
        None
    }
}

/// non_port_module_item ::=
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct NonPortModuleItem {}
