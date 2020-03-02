//! A.9.3 Identifiers
use crate::ast::*;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Identifier {
    pub token: TokenIndex,
}

impl Parse for Identifier {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Identifier::default();
        if let Some(token) = parser.peek() {
            if token.token == Token::Identifier {
                res.token = parser.index();
                parser.advance();
                return Some(res);
            } else {
                parser.err(
                    token.span.from,
                    token.span.to,
                    Message::UnexpectedToken(Token::Identifier, token.text.to_owned()),
                );
                return None;
            }
        } else {
            parser.err(
                parser.location(),
                parser.location(),
                Message::UnexpectedToken(Token::Identifier, "end of file".to_owned()),
            );
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut parser = Parser::from("abcdefg");
        let id = Identifier::parse(&mut parser);
        assert!(id.is_some());
        assert_eq!(id.unwrap().token, 0);
    }
}
