//! A.2.1 Declaration types

use crate::ast::*;

/// A.2.1.2 Port declarations
/// inout_declaration ::= inout net_port_type list_of_port_identifiers
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct InOutDeclaration {
    pub port_type: NetPortType,
}
