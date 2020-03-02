use crate::ast::*;
use crate::{lexer::Token, parser::Parser};
use serde::{Deserialize, Serialize};

/// A.1.2 SystemVerilog source text
/// module_declaration ::= module_ansi_header { module_item } endmodule
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleDeclaration {
    pub header: ModuleHeader,
}

/// A.1.2 SystemVerilog source text
/// module_ansi_header ::= { attribute_instance } module_keyword module_identifier [ list_of_port_declarations ] ;
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleHeader {
    pub attributes: attribute::Attributes,
    pub identifier: identifier::Identifier,
    pub ports: Ports,
}

/// A.1.3 Module parameters and ports
/// list_of_port_declarations ::= ( [ { attribute_instance } ansi_port_declaration { , { attribute_instance } ansi_port_declaration } ] )
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Ports {
    pub ports: Vec<(attribute::Attributes, Port)>,
}

/// A.1.3 Module parameters and ports
/// ansi_port_declaration ::= [ net_port_header ] port_identifier
/// net_port_header ::= [ port_direction ] net_port_type
/// net_port_type ::= [ net_type ] data_type_or_implicit
/// data_type_or_implicit ::= data_type
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Port {
    pub direction: Option<PortDirection>,
    pub port_type: Option<NetType>,
    pub data_type: Option<DataType>,
    pub identifier: identifier::Identifier,
}

/// A.1.3 Module parameters and ports
/// port_direction ::= input | output | inout | ref
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum PortDirection {
    Input,
    Output,
    InOut,
    Ref,
}

/// A.1.3 Module parameters and ports
/// net_type ::= supply0 | supply1 | tri | triand | trior | trireg | tri0 | tri1 | uwire| wire | wand | wor
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum NetType {
    Wire,
}

/// A.2.2.1 Net and variable types
/// data_type ::= integer_vector_type [ signing ] { packed_dimension }
/// signing ::= signed | unsigned
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct DataType {
    integer_type: IntegerVectorType,
    sign: Option<Signing>,
}

/// A.2.2.1 Net and variable types
/// integer_vector_type ::= bit | logic | reg
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum IntegerVectorType {
    Bit,
    Logic,
    Reg,
}

impl Default for IntegerVectorType {
    fn default() -> Self {
        Self::Bit
    }
}

/// A.2.2.1 Net and variable types
/// signing ::= signed | unsigned
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum Signing {
    Signed,
    Unsigned,
}

impl Parse for ModuleDeclaration {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = ModuleDeclaration::default();
        if let Some(header) = ModuleHeader::parse(parser) {
            res.header = header;
            // TODO: module_item
            if parser.probe_err(&[Token::EndModule]) {
                parser.advance();
                return Some(res);
            }
        }
        None
    }
}

impl Parse for ModuleHeader {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = ModuleHeader::default();
        if parser.probe(&[Token::LParen]) {
            if let Some(attrs) = attribute::Attributes::parse(parser) {
                res.attributes = attrs;
            }
        }
        if parser.probe_err(&[Token::Module]) {
            parser.advance();
            let identifier = identifier::Identifier::parse(parser)?;
            res.identifier = identifier;
            if parser.probe(&[Token::LParen]) {
                if let Some(ports) = Ports::parse(parser) {
                    res.ports = ports;
                }
            }
            if parser.probe_err(&[Token::Semicolon]) {
                parser.advance();
                return Some(res);
            }
        }
        None
    }
}

impl Parse for Ports {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Ports::default();
        if parser.probe_err(&[Token::LParen]) {
            parser.advance();
            loop {
                let attrs = if parser.probe(&[Token::LParen]) {
                    attribute::Attributes::parse(parser).unwrap_or_default()
                } else {
                    attribute::Attributes::default()
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
        ]) {
            res.port_type = NetType::parse(parser);
        }
        if parser.probe(&[Token::Bit, Token::Logic, Token::Reg]) {
            res.data_type = DataType::parse(parser);
        }

        if parser.probe_err(&[Token::Identifier]) {
            if let Some(identifier) = identifier::Identifier::parse(parser) {
                res.identifier = identifier;
                return Some(res);
            }
        }

        None
    }
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

impl Parse for DataType {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Self::default();
        if let Some(integer_type) = IntegerVectorType::parse(parser) {
            res.integer_type = integer_type;
            if parser.probe(&[Token::Signed, Token::Unsigned]) {
                res.sign = Signing::parse(parser);
            }
            Some(res)
        } else {
            None
        }
    }
}

impl Parse for IntegerVectorType {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(token) = parser.peek() {
            let res = match token.token {
                Token::Bit => Some(IntegerVectorType::Bit),
                Token::Logic => Some(IntegerVectorType::Logic),
                Token::Reg => Some(IntegerVectorType::Reg),
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

impl Parse for Signing {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(token) = parser.peek() {
            let res = match token.token {
                Token::Signed => Some(Signing::Signed),
                Token::Unsigned => Some(Signing::Unsigned),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module() {
        let mut parser = Parser::from("module test; endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        println!("{:?}", parser);
        println!("{:?}", m);
        assert_eq!(m.unwrap().header.identifier.token, 1);
    }
}
