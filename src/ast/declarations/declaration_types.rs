//! A.2.1 Declaration types

use crate::ast::*;

/// A.2.1.2 Port declarations
/// inout_declaration ::= inout net_port_type list_of_port_identifiers
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct InOutDeclaration {
    pub port_type: NetPortType,
    pub identifiers: ListOfPortIdentifiers,
}

impl Parse for InOutDeclaration {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if parser.probe(&[Token::InOut]) {
            parser.advance();
            if let Some(port_type) = NetPortType::parse(parser) {
                if let Some(identifiers) = ListOfPortIdentifiers::parse(parser) {
                    return Some(InOutDeclaration {
                        port_type,
                        identifiers,
                    });
                }
            }
        }
        None
    }
}

/// A.2.1.2 Port declarations
/// input_declaration ::= input net_port_type list_of_port_identifiers
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct InputDeclaration {
    pub port_type: NetPortType,
    pub identifiers: ListOfPortIdentifiers,
}

impl Parse for InputDeclaration {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if parser.probe(&[Token::Input]) {
            parser.advance();
            if let Some(port_type) = NetPortType::parse(parser) {
                if let Some(identifiers) = ListOfPortIdentifiers::parse(parser) {
                    return Some(InputDeclaration {
                        port_type,
                        identifiers,
                    });
                }
            }
        }
        None
    }
}

/// A.2.1.2 Port declarations
/// output_declaration ::= output net_port_type list_of_port_identifiers
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct OutputDeclaration {
    pub port_type: NetPortType,
    pub identifiers: ListOfPortIdentifiers,
}

impl Parse for OutputDeclaration {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if parser.probe(&[Token::Output]) {
            parser.advance();
            if let Some(port_type) = NetPortType::parse(parser) {
                if let Some(identifiers) = ListOfPortIdentifiers::parse(parser) {
                    return Some(OutputDeclaration {
                        port_type,
                        identifiers,
                    });
                }
            }
        }
        None
    }
}
