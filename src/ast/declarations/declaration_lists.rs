//! A.2.3 Declaration lists

use crate::ast::*;

/// list_of_port_identifiers ::= port_identifier { unpacked_dimension } { , port_identifier { unpacked_dimension } }
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ListOfPortIdentifiers {
    pub ports: Vec<(Identifier, Option<UnpackedDimension>)>,
}

impl Parse for ListOfPortIdentifiers {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(init_identifier) = Identifier::parse(parser) {
            let mut res = Self::default();
            let dimension = if parser.probe(&[Token::LBracket]) {
                UnpackedDimension::parse(parser)
            } else {
                None
            };
            res.ports.push((init_identifier, dimension));
            while parser.probe(&[Token::Comma]) {
                parser.advance();
                if let Some(identifier) = Identifier::parse(parser) {
                    let dimension = if parser.probe(&[Token::LBracket]) {
                        UnpackedDimension::parse(parser)
                    } else {
                        None
                    };
                    res.ports.push((identifier, dimension));
                }
            }
            return Some(res);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_of_port_identifiers() {
        let mut parser = Parser::from("abc[1:3],cd");
        let m = ListOfPortIdentifiers::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().ports.len(), 2);
    }
}
