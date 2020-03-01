use super::identifier::Identifier;
use crate::ast::{Parse, TokenIndex};
use crate::{lexer::Token, parser::Parser};
use serde::{Deserialize, Serialize};

// A.1.4 Module parameters and ports
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Ports {
    pub ports: Vec<Port>,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Port {
    pub ports: Vec<Port>,
}
