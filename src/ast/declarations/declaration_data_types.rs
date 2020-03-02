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

/// A.2.2.1 Net and variable types
/// implicit_data_type ::= [ signing ] { packed_dimension }
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ImplicitDataType {
    pub sign: Option<Signing>,
    pub dimensions: Vec<PackedDimension>,
}

impl Parse for ImplicitDataType {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Self::default();
        if parser.probe(&[Token::Signed, Token::Unsigned]) {
            res.sign = Signing::parse(parser);
        }
        while parser.probe(&[Token::LBracket]) {
            if let Some(dimension) = PackedDimension::parse(parser) {
                res.dimensions.push(dimension);
            }
        }
        Some(res)
    }
}

/// A.2.2.1 Net and variable types
/// data_type_or_implicit ::= data_type | implicit_data_type
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum DataTypeOrImplicit {
    Data(DataType),
    ImplicitData(ImplicitDataType),
}

impl Default for DataTypeOrImplicit {
    fn default() -> Self {
        Self::Data(DataType::default())
    }
}

impl Parse for DataTypeOrImplicit {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if parser.probe(&[Token::Bit, Token::Logic, Token::Reg]) {
            if let Some(data) = DataType::parse(parser) {
                return Some(DataTypeOrImplicit::Data(data));
            }
        }
        if parser.probe(&[
            Token::Signed,
            Token::Unsigned,
            Token::LBracket,
            // FOLLOW
            Token::Identifier,
        ]) {
            if let Some(data) = ImplicitDataType::parse(parser) {
                return Some(DataTypeOrImplicit::ImplicitData(data));
            }
        }
        None
    }
}

/// A.2.2.1 Net and variable types
/// net_port_type ::= [ net_type ] data_type_or_implicit
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct NetPortType {
    pub net_type: Option<NetType>,
    pub data_type_or_implicit: DataTypeOrImplicit,
}

impl Parse for NetPortType {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = NetPortType::default();
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
            res.net_type = NetType::parse(parser);
        }
        if parser.probe(&[
            Token::Bit,
            Token::Logic,
            Token::Reg,
            Token::Signed,
            Token::Unsigned,
            Token::LBracket,
            // FOLLOW
            Token::Identifier,
        ]) {
            if let Some(data) = DataTypeOrImplicit::parse(parser) {
                res.data_type_or_implicit = data;
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

    #[test]
    fn net_port_type() {
        let mut parser = Parser::from("logic");
        let m = NetPortType::parse(&mut parser);
        println!("{:?}", parser);
        assert_eq!(m.as_ref().unwrap().net_type, None);
        assert_eq!(
            m.as_ref().unwrap().data_type_or_implicit,
            DataTypeOrImplicit::Data(DataType {
                integer_type: IntegerVectorType::Logic,
                ..DataType::default()
            })
        );

        let mut parser = Parser::from("wire abc");
        let m = NetPortType::parse(&mut parser);
        println!("{:?}", parser);
        assert_eq!(m.as_ref().unwrap().net_type, Some(NetType::Wire));
    }
}
