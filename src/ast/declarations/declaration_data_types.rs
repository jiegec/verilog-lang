//! A.2.2 Declaration data types
use crate::ast::*;

/// A.2.2.1 Net and variable types
/// data_type ::= integer_vector_type [ signing ] { packed_dimension }
/// signing ::= signed | unsigned
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct DataType {
    pub integer_type: IntegerVectorType,
    pub sign: Option<Signing>,
    pub dimensions: Vec<PackedDimension>,
}

impl Parse for DataType {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Self::default();
        if let Some(integer_type) = IntegerVectorType::parse(parser) {
            res.integer_type = integer_type;
            if parser.probe(&[Token::Signed, Token::Unsigned]) {
                res.sign = Signing::parse(parser);
            }
            while parser.probe(&[Token::LBracket]) {
                if let Some(dimension) = PackedDimension::parse(parser) {
                    res.dimensions.push(dimension);
                } else {
                    break;
                }
            }
            Some(res)
        } else {
            None
        }
    }
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
    fn data_type() {
        let mut parser = Parser::from("logic signed [1:2][][]");
        let m = DataType::parse(&mut parser);
        println!("{:?}", parser);
        assert_eq!(m.as_ref().unwrap().integer_type, IntegerVectorType::Logic);
        assert_eq!(m.as_ref().unwrap().sign, Some(Signing::Signed));
        assert_eq!(
            m.as_ref().unwrap().dimensions[0]
                .from
                .as_ref()
                .unwrap()
                .token,
            3
        );
    }
}
