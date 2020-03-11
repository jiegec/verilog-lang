//! Parser

use crate::diagnostic::{Diagnostic, Message, Severity};
use crate::lexer::{Lexer, Location, ParsedToken, Span, Token};

#[derive(Debug)]
pub struct Parser<'a> {
    input: &'a str,
    index: usize,
    end_loc: Location,
    tokens: Vec<ParsedToken<'a>>,
    diag: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    pub fn from(input: &'a str) -> Parser<'a> {
        let lexer = Lexer::lex(input);
        Self::from_lexer(lexer)
    }

    pub fn from_lexer(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            input: lexer.input,
            index: 0,
            end_loc: lexer.loc,
            tokens: lexer.tokens,
            diag: lexer.diag,
        }
    }

    pub(crate) fn peek(&self) -> Option<ParsedToken<'a>> {
        if self.index < self.tokens.len() {
            Some(self.tokens[self.index])
        } else {
            None
        }
    }

    fn skip_comment(&mut self) {
        while self.index < self.tokens.len() && self.tokens[self.index].token == Token::Comment {
            self.index += 1;
        }
    }

    pub(crate) fn probe(&mut self, arr: &[Token]) -> bool {
        self.skip_comment();
        if self.index < self.tokens.len() {
            arr.contains(&self.tokens[self.index].token)
        } else {
            false
        }
    }

    pub(crate) fn probe_err(&mut self, arr: &[Token]) -> bool {
        self.skip_comment();
        let res = if self.index < self.tokens.len() {
            arr.contains(&self.tokens[self.index].token)
        } else {
            false
        };
        if !res {
            self.err(
                self.location_from(),
                self.location_to(),
                Message::UnexpectedTokens(arr.to_owned(), self.current_text()),
            );
        }
        res
    }

    pub(crate) fn advance(&mut self) {
        self.index += 1;
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn avail(&self) -> bool {
        self.index < self.tokens.len()
    }

    pub(crate) fn location_from(&self) -> Location {
        if self.index < self.tokens.len() {
            self.tokens[self.index].span.from
        } else {
            self.end_loc
        }
    }

    pub(crate) fn location_to(&self) -> Location {
        if self.index < self.tokens.len() {
            self.tokens[self.index].span.to
        } else {
            self.end_loc
        }
    }

    pub(crate) fn current_text(&self) -> String {
        if self.index < self.tokens.len() {
            self.tokens[self.index].text.to_owned()
        } else {
            "end of file".to_owned()
        }
    }

    pub(crate) fn err(&mut self, from: Location, to: Location, msg: Message) {
        self.diag.push(Diagnostic {
            pos: Span { from, to },
            msg,
            severity: Severity::Error,
        });
    }

    pub(crate) fn warn(&mut self, from: Location, to: Location, msg: Message) {
        self.diag.push(Diagnostic {
            pos: Span { from, to },
            msg,
            severity: Severity::Warning,
        });
    }

    pub fn get_diag(&self) -> &Vec<Diagnostic> {
        &self.diag
    }

    pub fn get_token(&self, index: usize) -> &ParsedToken {
        &self.tokens[index]
    }
}
