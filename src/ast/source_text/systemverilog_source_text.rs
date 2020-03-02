//! A.1.2 SystemVerilog source text
use crate::ast::*;

/// source_text ::= { description }
/// description ::= module_declaration
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct SourceText {
    pub modules: Vec<ModuleDeclaration>,
}

impl Parse for SourceText {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = SourceText::default();
        while parser.avail() {
            if let Some(module) = ModuleDeclaration::parse(parser) {
                res.modules.push(module);
            } else {
                break;
            }
        }
        Some(res)
    }
}

/// module_declaration ::= module_ansi_header { module_item } endmodule
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleDeclaration {
    pub header: ModuleHeader,
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

/// module_ansi_header ::= { attribute_instance } module_keyword module_identifier [ list_of_port_declarations ] ;
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleHeader {
    pub attributes: Attributes,
    pub identifier: Identifier,
    pub ports: Ports,
}

impl Parse for ModuleHeader {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = ModuleHeader::default();
        if parser.probe(&[Token::LParen]) {
            if let Some(attrs) = Attributes::parse(parser) {
                res.attributes = attrs;
            }
        }
        if parser.probe_err(&[Token::Module]) {
            parser.advance();
            let identifier = Identifier::parse(parser)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module() {
        let mut parser = Parser::from("module test; endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);

        let mut parser = Parser::from("module test(); endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports.len(), 0);

        let mut parser = Parser::from("module test(wire sig); endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        println!("{:?}", parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports.len(), 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports[0].1.direction, None);
        assert_eq!(
            m.as_ref().unwrap().header.ports.ports[0]
                .1
                .net_port_type
                .as_ref()
                .unwrap()
                .net_type,
            Some(NetType::Wire)
        );

        let mut parser = Parser::from("module test(logic sig, input sig2); endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports.len(), 2);
        assert_eq!(m.as_ref().unwrap().header.ports.ports[0].1.direction, None);
        assert_eq!(
            m.as_ref().unwrap().header.ports.ports[0]
                .1
                .net_port_type
                .as_ref()
                .unwrap()
                .data_type_or_implicit,
            DataTypeOrImplicit::Data(DataType {
                integer_type: IntegerVectorType::Logic,
                ..DataType::default()
            })
        );
        assert_eq!(
            m.as_ref().unwrap().header.ports.ports[1].1.direction,
            Some(PortDirection::Input)
        );
    }
}
