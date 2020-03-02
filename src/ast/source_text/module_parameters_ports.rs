//! A.1.3 Module parameters and ports
use crate::ast::*;

/// list_of_port_declarations ::= ( [ { attribute_instance } ansi_port_declaration { , { attribute_instance } ansi_port_declaration } ] )
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Ports {
    pub ports: Vec<(Attributes, Port)>,
}

impl Parse for Ports {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Ports::default();
        if parser.probe_err(&[Token::LParen]) {
            parser.advance();
            loop {
                let attrs = if parser.probe(&[Token::LParen]) {
                    Attributes::parse(parser).unwrap_or_default()
                } else {
                    Attributes::default()
                };
                if let Some(port) = Port::parse(parser) {
                    res.ports.push((attrs, port));
                }
                if parser.probe(&[Token::Comma]) {
                    parser.advance();
                    continue;
                } else if parser.probe_err(&[Token::Comma, Token::RParen]) {
                    parser.advance();
                    break;
                } else {
                    return None;
                }
            }
        }
        Some(res)
    }
}

/// ansi_port_declaration ::= [ net_port_header ] port_identifier { unpacked_dimension }
/// net_port_header ::= [ port_direction ] net_port_type
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Port {
    pub direction: Option<PortDirection>,
    pub net_port_type: Option<NetPortType>,
    pub identifier: Identifier,
    pub dimensions: Vec<UnpackedDimension>,
}

impl Parse for Port {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Port::default();
        if parser.probe(&[Token::Input, Token::Output, Token::InOut, Token::Ref]) {
            res.direction = PortDirection::parse(parser);
        }
        if parser.probe(&[
            Token::Supply0,
            Token::Supply1,
            Token::Tri,
            Token::TriAnd,
            Token::TriOr,
            Token::TriReg,
            Token::Tri0,
            Token::Tri1,
            Token::Uwire,
            Token::Wire,
            Token::Wand,
            Token::Wor,
            // data_type_or_implicit
            Token::Bit,
            Token::Logic,
            Token::Reg,
            Token::Signed,
            Token::Unsigned,
            Token::LBracket,
        ]) {
            res.net_port_type = NetPortType::parse(parser);
        }

        if parser.probe_err(&[Token::Identifier]) {
            if let Some(identifier) = Identifier::parse(parser) {
                res.identifier = identifier;
                while parser.probe(&[Token::LBracket]) {
                    if let Some(dimension) = UnpackedDimension::parse(parser) {
                        res.dimensions.push(dimension);
                    }
                }
                return Some(res);
            }
        }

        None
    }
}

/// port_direction ::= input | output | inout | ref
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum PortDirection {
    Input,
    Output,
    InOut,
    Ref,
}

impl Parse for PortDirection {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(token) = parser.peek() {
            let res = match token.token {
                Token::Input => Some(PortDirection::Input),
                Token::Output => Some(PortDirection::Output),
                Token::InOut => Some(PortDirection::InOut),
                Token::Ref => Some(PortDirection::Ref),
                _ => None,
            };
            if res.is_some() {
                parser.advance();
            }
            res
        } else {
            None
        }
    }
}

/// net_type ::= supply0 | supply1 | tri | triand | trior | trireg | tri0 | tri1 | uwire| wire | wand | wor
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum NetType {
    Wire,
}

impl Parse for NetType {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(token) = parser.peek() {
            let res = match token.token {
                Token::Wire => Some(NetType::Wire),
                _ => None,
            };
            if res.is_some() {
                parser.advance();
            }
            res
        } else {
            None
        }
    }
}

/// port_declaration ::= { attribute_instance } inout_declaration | { attribute_instance } input_declaration | { attribute_instance } output_declaration | { attribute_instance } ref_declaration | { attribute_instance } interface_port_declaration
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum PortDeclaration {
    InOut(Attributes),
    Input(Attributes),
    Output(Attributes),
    Ref(Attributes),
    InterfacePort,
}

impl Parse for PortDeclaration {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports() {
        let mut parser = Parser::from("(logic [1:2] sig, input wire [3:4] sig2)");
        let m = Ports::parse(&mut parser);
        println!("{:?}", parser);
        println!("{:?}", m);
        assert_eq!(m.as_ref().unwrap().ports.len(), 2);
        assert_eq!(m.as_ref().unwrap().ports[0].1.direction, None);
    }
}
