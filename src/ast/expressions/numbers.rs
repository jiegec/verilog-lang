//! A.8.7 Numbers

use crate::ast::*;

/// number
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Number {
    pub token: TokenIndex,
}

impl Parse for Number {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Number::default();
        if let Some(token) = parser.peek() {
            if token.token == Token::Number {
                res.token = parser.index();
                parser.advance();
                return Some(res);
            } else {
                parser.err(
                    token.span.from,
                    token.span.to,
                    Message::UnexpectedToken(Token::Number, token.text.to_owned()),
                );
                return None;
            }
        } else {
            parser.err(
                parser.location(),
                parser.location(),
                Message::UnexpectedToken(Token::Number, "end of file".to_owned()),
            );
        }
        None
    }
}
