//! A.2.5 Declaration ranges

use crate::ast::*;

/// packed_dimension ::= [ constant_range ] | unsized_dimension
/// unsized_dimension ::= [ ]
/// constant_range ::= constant_expression : constant_expression
/// constant_expression ::= constant_primary
/// constant_primary ::= primary_literal
/// primary_literal ::= number
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct PackedDimension {
    pub from: Option<Number>,
    pub to: Option<Number>,
}

impl Parse for PackedDimension {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if parser.probe_err(&[Token::LBracket]) {
            parser.advance();
            if parser.probe(&[Token::Number]) {
                let mut res = Self::default();
                res.from = Number::parse(parser);
                if parser.probe_err(&[Token::Colon]) {
                    parser.advance();
                    res.to = Number::parse(parser);
                    if parser.probe_err(&[Token::RBracket]) {
                        parser.advance();
                        return Some(res);
                    }
                }
            } else if parser.probe_err(&[Token::RBracket]) {
                return Some(PackedDimension {
                    from: None,
                    to: None,
                });
            }
        }
        None
    }
}

/// unpacked_dimension ::= [ constant_range ]
/// constant_range ::= constant_expression : constant_expression
/// constant_expression ::= constant_primary
/// constant_primary ::= primary_literal
/// primary_literal ::= number
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct UnpackedDimension {
    pub from: Option<Number>,
    pub to: Option<Number>,
}

impl Parse for UnpackedDimension {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if parser.probe_err(&[Token::LBracket]) {
            parser.advance();
            if parser.probe(&[Token::Number]) {
                let mut res = Self::default();
                res.from = Number::parse(parser);
                if parser.probe_err(&[Token::Colon]) {
                    parser.advance();
                    res.to = Number::parse(parser);
                    if parser.probe_err(&[Token::RBracket]) {
                        parser.advance();
                        return Some(res);
                    }
                }
            } else if parser.probe_err(&[Token::RBracket]) {
                return Some(UnpackedDimension {
                    from: None,
                    to: None,
                });
            }
        }
        None
    }
}
