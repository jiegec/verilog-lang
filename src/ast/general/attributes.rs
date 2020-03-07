//! A.9.1 Attributes

use crate::ast::*;
use crate::{lexer::Token, parser::Parser};
use serde::{Deserialize, Serialize};

/// ## { attribute_instance }
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Attributes {
    pub attrs: Vec<Attribute>,
}

impl Parse for Attributes {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Self::default();
        while parser.probe(&[Token::LParen]) {
            if let Some(attr) = Attribute::parse(parser) {
                res.attrs.push(attr);
            }
        }
        Some(res)
    }
}

/// ## attribute_instance ::= (* attr_spec { , attr_spec } *)
/// ## attr_spec ::= attr_name
/// ## attr_name ::= identifier
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Attribute {
    pub attrs: Vec<Identifier>,
}

impl Parse for Attribute {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(token1) = parser.peek() {
            if token1.token == Token::LParen {
                parser.advance();
                if let Some(token2) = parser.peek() {
                    if token2.token == Token::OpMultiply {
                        parser.advance();
                        let mut res = Attribute::default();
                        loop {
                            if parser.probe(&[Token::Identifier]) {
                                let id = Identifier::parse(parser)?;
                                res.attrs.push(id);
                                // TODO: handle attr_name = constant_expression
                                if parser.probe(&[Token::Comma]) {
                                    parser.advance();
                                }
                            } else if parser.probe(&[Token::OpMultiply]) {
                                parser.advance();
                                if parser.probe(&[Token::RParen]) {
                                    // TODO: error msg
                                }
                                parser.advance();
                                return Some(res);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut parser = Parser::from("(* mark_debug *)");
        let attr = Attribute::parse(&mut parser);
        assert!(attr.is_some());
        assert_eq!(attr.unwrap().attrs[0].token, 2);

        let mut parser = Parser::from("(* mark_debug, debug_mark *)");
        let attr = Attribute::parse(&mut parser);
        assert!(attr.is_some());
        let attr = attr.unwrap();
        assert_eq!(attr.attrs[0].token, 2);
        assert_eq!(attr.attrs[1].token, 4);
    }
}
