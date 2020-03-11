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
            if parser.probe(&[Token::Module, Token::MacroModule]) {
                if let Some(module) = ModuleDeclaration::parse(parser) {
                    res.modules.push(module);
                } else {
                    break;
                }
            } else {
                parser.advance();
            }
        }
        Some(res)
    }
}

/// module_declaration ::= module_ansi_header { module_item } endmodule
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleDeclaration {
    pub header: ModuleHeader,
    pub items: Vec<ModuleItem>,
}

impl Parse for ModuleDeclaration {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = ModuleDeclaration::default();
        if let Some(header) = ModuleHeader::parse(parser) {
            res.header = header;
            // TODO: module_item
            while !parser.probe(&[Token::EndModule]) && parser.avail() {
                if let Some(item) = ModuleItem::parse(parser) {
                    res.items.push(item);
                } else {
                    parser.advance();
                }
            }
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
    fn simple_module() {
        let mut parser = Parser::from("module test; endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);

        let mut parser = Parser::from("module test(); endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports.len(), 0);
    }

    #[test]
    fn module_signals() {
        let mut parser = Parser::from("module test(wire sig); endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
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

        let mut parser = Parser::from("module test(output reg[6:0] sig2); endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports.len(), 1);
        assert_eq!(
            m.as_ref().unwrap().header.ports.ports[0].1.direction,
            Some(PortDirection::Output)
        );
        assert_eq!(
            m.as_ref().unwrap().header.ports.ports[0]
                .1
                .net_port_type
                .as_ref()
                .unwrap()
                .data_type_or_implicit,
            DataTypeOrImplicit::Data(DataType {
                integer_type: IntegerVectorType::Reg,
                dimensions: vec![PackedDimension {
                    from: Some(Number { token: 6 }),
                    to: Some(Number { token: 8 })
                }],
                ..DataType::default()
            })
        );
        assert_eq!(m.as_ref().unwrap().items.len(), 0);
    }

    #[test]
    fn module() {
        let mut parser =
            Parser::from("module test(logic sig, input sig2); output wire [1:0] test; endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports.len(), 2);

        let mut parser =
            Parser::from("module test(logic sig, input sig2); output wire [1:0] test; endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().header.identifier.token, 1);
        assert_eq!(m.as_ref().unwrap().header.ports.ports.len(), 2);
        assert_eq!(m.as_ref().unwrap().items.len(), 1);

        let mut parser = Parser::from("module test; output wire [1:0] test; begin end endmodule");
        let m = ModuleDeclaration::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().items.len(), 1);
    }

    #[test]
    fn source_text_comments() {
        let mut parser = Parser::from("// some comment");
        let m = SourceText::parse(&mut parser);
        assert_eq!(parser.get_diag().len(), 0);
    }

    #[test]
    fn source_text_multiple_modules() {
        let mut parser = Parser::from("module a;endmodule module b;endmodule");
        let m = SourceText::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().modules.len(), 2);
    }

    #[test]
    fn netlist() {
        let mut parser = Parser::from(
            r#"
module mod_top (
    signal1,
    signal1);
input signal1;
output [31:0] signal2;

wire gnd;
wire vcc;
wire unknown;

assign gnd = 1'b0;
assign vcc = 1'b1;
assign unknown = 1'bx;

tri1 devclrn;
tri1 devpor;
tri1 devoe;

wire \Add0~6_combout;

cycloneii_lcell_comb \Add0~6 (
    .dataa(signal1),
    .datab(vcc),
    .datac(vcc),
    .datad(vcc),
    .cin(\Add0~5),
    .combout(\Add0~6_combout),
    .cout(\Add0~7));
defparam \Add0~6 .lut_mask = 16habab;
defparam \Add0~6 .sum_lutc_input = "cin";
endmodule "#,
        );
        let m = SourceText::parse(&mut parser);
        assert_eq!(m.as_ref().unwrap().modules.len(), 1);
        assert_eq!(parser.get_diag().len(), 0);
    }
}
