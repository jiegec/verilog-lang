//! A.2.1 Declaration types

use crate::ast::*;

macro_rules! port_declaration {
    ($(#[$outer:meta])* $s:ident, $tok:ident, $name:ident) => {
        $(#[$outer])*
        #[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
        pub struct $s {
            pub port_type: NetPortType,
            pub identifiers: ListOfPortIdentifiers,
        }

        impl Parse for $s {
            fn parse(parser: &mut Parser<'_>) -> Option<Self> {
                if parser.probe(&[Token::$tok]) {
                    parser.advance();
                    if let Some(port_type) = NetPortType::parse(parser) {
                        if let Some(identifiers) = ListOfPortIdentifiers::parse(parser) {
                            return Some($s {
                                port_type,
                                identifiers,
                            });
                        }
                    }
                }
                None
            }
        }
    };
}

port_declaration! {
    /// # A.2.1.2 Port declarations
    /// ## inout_declaration ::= inout net_port_type list_of_port_identifiers
    InOutDeclaration, InOut, inout
}

port_declaration! {
    /// # A.2.1.2 Port declarations
    /// ## input_declaration ::= input net_port_type list_of_port_identifiers
    InputDeclaration, Input, input
}

port_declaration! {
    /// # A.2.1.2 Port declarations
    /// ## output_declaration ::= output net_port_type list_of_port_identifiers
    OutputDeclaration, Output, output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inout_declaration() {
        let mut parser = Parser::from("inout wire [2:3] signal");
        let m = InOutDeclaration::parse(&mut parser);
        assert_eq!(
            m.as_ref().unwrap().port_type.net_type.as_ref().unwrap(),
            &NetType::Wire
        );
    }
}
