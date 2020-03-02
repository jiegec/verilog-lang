use crate::diagnostic::{Diagnostic, Message, Severity};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strcursor::StrCursor;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Span {
    pub from: Location,
    pub to: Location,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Token {
    // Types
    Number,
    Identifier,
    StringLiteral,
    Directive,
    Comment,

    // Keywords, Annex B
    AcceptOn,
    Alias,
    Always,
    AlwaysComb,
    AlwaysFf,
    AlwaysLatch,
    And,
    Assert,
    Assign,
    Automatic,
    Before,
    Begin,
    Bind,
    Bins,
    BinsOf,
    Bit,
    Break,
    Buf,
    BufIf0,
    BufIf1,
    Byte,
    Case,
    Casex,
    Casez,
    Cell,
    CHandle,
    Checker,
    Class,
    Clocking,
    Cmos,
    Config,
    Const,
    Constraint,
    Context,
    Continue,
    Cover,
    CoverGroup,
    CoverPoint,
    Cross,
    Deassign,
    Default,
    DefParam,
    Design,
    Disable,
    Dist,
    Do,
    Edge,
    Else,
    End,
    EndCase,
    EndChecker,
    EndClass,
    EndClocking,
    EndConfig,
    EndFunction,
    EndGenerate,
    EndGroup,
    EndInterface,
    EndModule,
    EndPackage,
    EndPrimitive,
    EndProgram,
    EndProperty,
    EndSpecify,
    EndSequence,
    EndTable,
    EndTask,
    Enum,
    Event,
    Eventually,
    Expect,
    Export,
    Extends,
    Extern,
    Final,
    FirstMatch,
    For,
    Force,
    Forever,
    Fork,
    ForkJoin,
    Function,
    Generate,
    GenVar,
    Global,
    HighZ0,
    HighZ1,
    If,
    Iff,
    IfNone,
    IgnoreBins,
    IllegalBins,
    Implements,
    Implies,
    Import,
    IncDir,
    Include,
    Initial,
    InOut,
    Input,
    Inside,
    Instance,
    Integer,
    Interconnect,
    Interface,
    Intersect,
    Join,
    JoinAny,
    JoinNone,
    Large,
    Let,
    LibList,
    Library,
    Localparam,
    Logic,
    LongInt,
    MacroModule,
    Matches,
    Medium,
    ModPort,
    Module,
    Nand,
    NegEdge,
    NetType,
    New,
    NextTime,
    Nmos,
    Nor,
    NoShowCancelled,
    Not,
    NotIf0,
    NotIf1,
    Or,
    Output,
    Package,
    Packed,
    Parameter,
    Pmos,
    PosEdge,
    Primitive,
    Priority,
    Program,
    Property,
    Protected,
    Pull0,
    Pull1,
    PullDown,
    PullUp,
    PulseStyleOnDetect,
    PulseStyleOnEvent,
    Pure,
    Rand,
    RandC,
    RandCase,
    RandSequence,
    Rcmos,
    Real,
    Realtime,
    Ref,
    Reg,
    RejectOn,
    Release,
    Repeat,
    Restrict,
    Return,
    Rnmos,
    Rpmos,
    Rtran,
    RtranIf0,
    RtranIf1,
    SAlways,
    SEventually,
    SNextTime,
    SUntil,
    SUntilWith,
    Scalared,
    Sequence,
    ShortInt,
    ShortReal,
    ShowCancelled,
    Signed,
    Small,
    Soft,
    Solve,
    Specify,
    Specparam,
    Static,
    String,
    Strong,
    Strong0,
    Strong1,
    Struct,
    Super,
    Supply0,
    Supply1,
    SyncAcceptOn,
    SyncRejectOn,
    Table,
    Tagged,
    Task,
    This,
    Throughout,
    Time,
    TimePrecision,
    TimeUnit,
    Tran,
    TranIf0,
    TranIf1,
    Tri,
    Tri0,
    Tri1,
    TriAnd,
    TriOr,
    TriReg,
    Type,
    TypeDef,
    Union,
    Unique,
    Unique0,
    Unsigned,
    Until,
    UntilWith,
    Untyped,
    Use,
    Uwire,
    Var,
    Vectored,
    Virtual,
    Void,
    Wait,
    WaitOrder,
    Wand,
    Weak0,
    Weak1,
    While,
    Wildcard,
    Wire,
    With,
    Within,
    Wor,
    Xnor,
    Xor,

    // Delimiter
    Sharp,  // #
    LParen, // ()
    RParen,
    LBracket, // []
    RBracket,
    LBraces, // {}
    RBraces,
    Colon,     // :
    Comma,     // ,
    Semicolon, // ;
    Dot,       // .
    Equal,     // =
    At,        // @
    Question,  // ?

    // Operators, Table 9
    OpPlus,            // +
    OpMinus,           // -
    OpMultiply,        // *
    OpDivide,          // /
    OpPow,             // **
    OpMod,             // %
    OpGreaterThan,     // >
    OpGreaterEqual,    // >=
    OpLessThan,        // <
    OpLessEqual,       // <=
    OpNot,             // !
    OpAnd,             // &&
    OpOr,              // ||
    OpEqual,           // ==
    OpInequal,         // !=
    OpCaseEqual,       // ===
    OpCaseInequal,     // !==
    OpBitNeg,          // ~
    OpBitAnd,          // &
    OpBitOr,           // |
    OpBitXor,          // ^
    OpBitEquiv1,       // ^~
    OpBitEquiv2,       // ~^
    OpNand,            // ~&
    OpNor,             // ~|
    OpLeftShift,       // <<
    OpRightShift,      // >>
    OpArithLeftShift,  // <<<
    OpArithRightShift, // >>>
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ParsedToken<'a> {
    pub span: Span,
    pub token: Token,
    pub text: &'a str,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Lexer<'a> {
    cursor: StrCursor<'a>,
    pub(crate) loc: Location,
    pub(crate) input: &'a str,
    pub tokens: Vec<ParsedToken<'a>>,
    pub diag: Vec<Diagnostic>,
}

fn keyword_map() -> HashMap<String, Token> {
    use Token::*;
    let mut map = HashMap::new();
    // TODO: all keywords
    map.insert(format!("always"), Always);
    map.insert(format!("always_comb"), AlwaysComb);
    map.insert(format!("always_ff"), AlwaysFf);
    map.insert(format!("and"), And);
    map.insert(format!("assign"), Assign);
    map.insert(format!("automatic"), Automatic);
    map.insert(format!("endmodule"), EndModule);
    map.insert(format!("module"), Module);
    map
}

impl<'a> Lexer<'a> {
    pub fn lex(input: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            input,
            cursor: StrCursor::new_at_start(input),
            loc: Location { row: 0, col: 0 },
            tokens: vec![],
            diag: vec![],
        };
        lexer.work();
        lexer
    }

    fn err(&mut self, from: Location, to: Location, msg: Message) {
        self.diag.push(Diagnostic {
            pos: Span { from, to },
            msg,
            severity: Severity::Error,
        });
    }

    fn warn(&mut self, from: Location, to: Location, msg: Message) {
        self.diag.push(Diagnostic {
            pos: Span { from, to },
            msg,
            severity: Severity::Warning,
        });
    }

    // A.9.2 Comments
    fn comment(&mut self) -> bool {
        let orig_cursor = self.cursor;
        if let Some((gc1, next)) = self.cursor.next() {
            if gc1.base_char() == '/' {
                if let Some((gc2, mut cursor)) = next.next() {
                    if gc2.base_char() == '/' {
                        // one line comment
                        let from = self.loc;
                        self.loc.col += 1;
                        let mut to = self.loc;
                        self.loc.col += 1;
                        while let Some((gc, next)) = cursor.next() {
                            if gc.base_char() == '\n' {
                                break;
                            }
                            to = self.loc;
                            self.loc.col += 1;
                            cursor = next;
                        }

                        // end of line
                        self.tokens.push(ParsedToken {
                            token: Token::Comment,
                            span: Span { from, to },
                            text: orig_cursor.slice_between(cursor).unwrap(),
                        });
                        self.cursor = cursor;
                        return true;
                    } else if gc2.base_char() == '*' {
                        // multi line comment
                        let from = self.loc;
                        self.loc.col += 2;
                        let mut prev_ch = ' ';
                        while let Some((gc, next)) = cursor.next() {
                            if gc.base_char() == '/' && prev_ch == '*' {
                                // end of comment
                                self.tokens.push(ParsedToken {
                                    token: Token::Comment,
                                    span: Span { from, to: self.loc },
                                    text: orig_cursor.slice_between(next).unwrap(),
                                });
                                self.cursor = next;
                                return true;
                            } else if gc.base_char() == '\n' {
                                self.loc.row += 1;
                                self.loc.col = 0;
                            } else {
                                self.loc.col += 1;
                            }
                            prev_ch = gc.base_char();
                            cursor = next;
                        }

                        // not closed until end of input
                        let to = Location {
                            row: self.loc.row,
                            col: self.loc.col - 1,
                        };
                        self.tokens.push(ParsedToken {
                            token: Token::Comment,
                            span: Span { from, to },
                            text: orig_cursor.slice_between(cursor).unwrap(),
                        });
                        self.err(from, self.loc, Message::MultilineCommentUnclosed);
                        self.cursor = cursor;
                        return true;
                    }
                }
            }
        }
        false
    }

    // A.8.7 Numbers
    fn number(&mut self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!(
                "^({}|{}|{}|{}|{}|{}|{}|{}|{})",
                // octal_number
                "([1-9][0-9_]*)?'[sS]?[oO][0-7xXzZ][0-7xXzZ_]*", // [ size ] octal_base octal_value
                // binary_number
                "([1-9][0-9_]*)?'[sS]?[bB][01xXzZ][01xXzZ_]*", // [ size ] binary_base binary_value
                // hex_number
                "([1-9][0-9_]*)?'[sS]?[hH][0-9a-fA-FxXzZ][0-9a-fA-FxXzZ_]*", // [ size ] hex_base hex_value
                // real_number
                "[0-9][0-9_]*(.[0-9][0-9_]*)?[eE][+-]?[0-9][0-9_]*", // unsigned_number [ . unsigned_number ] exp [ sign ] unsigned_number
                "[0-9][0-9_]*.[0-9][0-9_]*", // unsigned_number . unsigned_number
                // decimal_number
                "([1-9][0-9_]*)?'[sS]?[dD][0-9][0-9_]*", // [ size ] decimal_base unsigned_number
                "([1-9][0-9_]*)?'[sS]?[dD][xX]_*",       // [ size ] decimal_base x_digit { _ }
                "([1-9][0-9_]*)?'[sS]?[dD][zZ?]_*",      // [ size ] decimal_base z_digit { _ }
                "[0-9][0-9_]*",                         // unsigned_number
            ))
            .unwrap();
        }
        let s = self.cursor.slice_after();
        if let Some(m) = RE.find(s) {
            assert_eq!(m.start(), 0);
            let from = self.loc;
            let orig_pos = self.cursor.byte_pos();
            let orig_cursor = self.cursor;
            let new_pos = orig_pos + m.end();
            let new_cursor = StrCursor::new_at_left_of_byte_pos(self.input, new_pos);
            let mut cursor = self.cursor;
            while let Some((_, next)) = cursor.next() {
                if next == new_cursor {
                    self.tokens.push(ParsedToken {
                        token: Token::Comment,
                        span: Span { from, to: self.loc },
                        text: orig_cursor.slice_between(next).unwrap(),
                    });
                    self.loc.col += 1;
                    self.cursor = next;
                    return true;
                }
                self.loc.col += 1;
                cursor = next;
            }
        }
        false
    }

    // A.8.6 Operators
    fn operator(&mut self) -> bool {
        let mut first = ' ';
        let mut second = ' ';
        let mut third = ' ';
        if let Some((gc, next)) = self.cursor.next() {
            first = gc.base_char();
            if let Some((gc, next)) = next.next() {
                second = gc.base_char();
                if let Some((gc, _next)) = next.next() {
                    third = gc.base_char();
                }
            }
        }
        let (token, len) = match (first, second, third) {
            ('=', '=', '=') => (Token::OpCaseEqual, 3),
            ('!', '=', '=') => (Token::OpCaseInequal, 3),
            ('>', '>', '>') => (Token::OpArithRightShift, 3),
            ('<', '<', '<') => (Token::OpArithLeftShift, 3),
            ('~', '&', _) => (Token::OpNand, 2),
            ('~', '|', _) => (Token::OpNor, 2),
            ('~', '^', _) => (Token::OpBitEquiv2, 2),
            ('^', '~', _) => (Token::OpBitEquiv1, 2),
            ('=', '=', _) => (Token::OpEqual, 2),
            ('!', '=', _) => (Token::OpInequal, 2),
            ('&', '&', _) => (Token::OpAnd, 2),
            ('|', '|', _) => (Token::OpOr, 2),
            ('*', '*', _) => (Token::OpPow, 2),
            ('<', '=', _) => (Token::OpLessEqual, 2),
            ('>', '=', _) => (Token::OpGreaterEqual, 2),
            ('>', '>', _) => (Token::OpRightShift, 2),
            ('<', '<', _) => (Token::OpLeftShift, 2),
            ('+', _, _) => (Token::OpPlus, 1),
            ('-', _, _) => (Token::OpMinus, 1),
            ('!', _, _) => (Token::OpNot, 1),
            ('&', _, _) => (Token::OpBitAnd, 1),
            ('|', _, _) => (Token::OpBitOr, 1),
            ('^', _, _) => (Token::OpBitXor, 1),
            ('*', _, _) => (Token::OpMultiply, 1),
            ('/', _, _) => (Token::OpDivide, 1),
            ('%', _, _) => (Token::OpMod, 1),
            ('<', _, _) => (Token::OpLessThan, 1),
            ('>', _, _) => (Token::OpGreaterThan, 1),
            ('~', _, _) => (Token::OpBitNeg, 1),
            _ => {
                return false;
            }
        };
        let to = Location {
            row: self.loc.row,
            col: self.loc.col + len - 1,
        };
        let mut cursor = self.cursor;
        for _ in 0..len {
            cursor = cursor.next().unwrap().1;
        }
        self.tokens.push(ParsedToken {
            span: Span { from: self.loc, to },
            token,
            text: self.cursor.slice_between(cursor).unwrap(),
        });
        self.cursor = cursor;
        self.loc.col += len;
        true
    }

    // 2.6 Strings
    fn string(&mut self) -> bool {
        if let Some((gc, next)) = self.cursor.next() {
            if gc.base_char() == '"' {
                let mut cursor = next;
                let mut escaping = false;
                let from = self.loc;
                let mut loc = self.loc;
                let mut escape_loc = Location { row: 0, col: 0 };
                loc.col += 1;
                while let Some((gc, next)) = cursor.next() {
                    match (gc.base_char(), escaping) {
                        ('\\', false) => {
                            escaping = true;
                            escape_loc = loc;
                        }
                        ('n', true) | ('t', true) | ('\\', true) | ('"', true) => {
                            escaping = false;
                        }
                        (ch, true) if ch >= '0' && ch <= '7' => {
                            escaping = false;
                        }
                        (ch, true) => {
                            // bad escape character
                            self.warn(escape_loc, loc, Message::UnrecognizedEscapeCharacter(ch));
                            escaping = false;
                        }
                        ('"', false) => {
                            // end
                            self.tokens.push(ParsedToken {
                                span: Span { from, to: loc },
                                token: Token::StringLiteral,
                                text: self.cursor.slice_between(next).unwrap(),
                            });
                            self.cursor = next;
                            self.loc = loc;
                            self.loc.col += 1;
                            return true;
                        }
                        _ => {}
                    }
                    loc.col += 1;
                    cursor = next;
                }
            }
        }
        false
    }

    // 2.7 Identifiers, keywords, and system names
    fn identifier_keyword(&mut self) -> bool {
        // TODO: Escaped Identifiers
        lazy_static! {
            static ref KEYWORD: HashMap<String, Token> = keyword_map();
        }
        let mut cursor = self.cursor;
        let from = self.loc;
        let mut loc = self.loc;
        while let Some((gc, next)) = cursor.next() {
            match gc.base_char() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '$' | '_' => {
                    cursor = next;
                    loc.col += 1;
                }
                _ => {
                    // end
                    break;
                }
            }
        }

        // end of input
        let slice = self.cursor.slice_between(cursor).unwrap();

        self.loc = loc;
        loc.col -= 1;
        self.cursor = cursor;
        let token = *KEYWORD.get(slice).unwrap_or(&Token::Identifier);

        self.tokens.push(ParsedToken {
            span: Span { from, to: loc },
            token,
            text: slice,
        });
        true
    }

    fn delimiter(&mut self) -> bool {
        if let Some((gc, next)) = self.cursor.next() {
            let token = match gc.base_char() {
                '#' => Token::Sharp,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                '{' => Token::LBraces,
                '}' => Token::RBraces,
                ':' => Token::Colon,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '.' => Token::Dot,
                '=' => Token::Equal,
                '@' => Token::At,
                '?' => Token::Question,
                _ => return false,
            };
            self.tokens.push(ParsedToken {
                span: Span {
                    from: self.loc,
                    to: self.loc,
                },
                token,
                text: self.cursor.slice_between(next).unwrap(),
            });
            self.cursor = next;
            self.loc.col += 1;
            return true;
        }
        false
    }

    // 2.7.5 Compiler directives
    fn directive(&mut self) -> bool {
        if let Some((gc, next)) = self.cursor.next() {
            if gc.base_char() == '`' {
                self.loc.col += 1;
                let mut cursor = next;
                let from = self.loc;
                let mut loc = self.loc;
                while let Some((gc, next)) = cursor.next() {
                    match gc.base_char() {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '$' | '_' => {
                            cursor = next;
                            loc.col += 1;
                        }
                        _ => {
                            // end
                            break;
                        }
                    }
                }

                // end of input
                let slice = self.cursor.slice_between(cursor).unwrap();

                self.loc = loc;
                loc.col -= 1;
                self.cursor = cursor;

                self.tokens.push(ParsedToken {
                    span: Span { from, to: loc },
                    token: Token::Directive,
                    text: slice,
                });
                return true;
            }
        }
        false
    }

    fn work(&mut self) {
        while let Some((gc, next)) = self.cursor.next() {
            match gc.base_char() {
                ch @ _ if ch.is_whitespace() => {
                    if ch == '\n' {
                        self.loc.row += 1;
                        self.loc.col = 0;
                        self.cursor = next;
                        continue;
                    }
                }
                '/' if self.comment() => {
                    continue;
                }
                '0'..='9' | '\'' if self.number() => {
                    continue;
                }
                '+' | '-' | '!' | '~' | '&' | '|' | '^' | '*' | '/' | '%' | '=' | '<' | '>'
                    if self.operator() =>
                {
                    continue;
                }
                '"' if self.string() => {
                    continue;
                }
                'a'..='z' | 'A'..='Z' | '_' if self.identifier_keyword() => {
                    continue;
                }
                '#' | '(' | ')' | '[' | ']' | '{' | '}' | ':' | ',' | ';' | '.' | '=' | '@'
                | '?'
                    if self.delimiter() =>
                {
                    continue;
                }
                '`' if self.directive() => {
                    continue;
                }
                _ => {
                    self.err(self.loc, self.loc, Message::UnexpectedChar(gc.base_char()));
                }
            }
            self.loc.col += 1;
            self.cursor = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unexpected_char() {
        let lexer = Lexer::lex("é和é是不一样的");
        println!("{:?}", lexer);
        assert!(lexer.diag.len() > 0);
    }

    #[test]
    fn comment() {
        let lexer = Lexer::lex("// woc woc\nsomething // abcde");
        println!("{:?}", lexer.tokens);
        assert!(lexer.tokens.len() > 0);
        assert_eq!(lexer.tokens[0].text, "// woc woc");

        let lexer = Lexer::lex("/* woc woc\nsomething */");
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 1);
        assert_eq!(lexer.tokens[0].text, "/* woc woc\nsomething */");
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 1, col: 11 });

        let lexer = Lexer::lex("/* not closed\ncomment ");
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens[0].text, "/* not closed\ncomment ");
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 1, col: 7 });
        assert_eq!(lexer.diag.len(), 1);
    }

    #[test]
    fn number() {
        let lexer = Lexer::lex("1234");
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 1);
        assert_eq!(lexer.tokens[0].text, "1234");
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 3 });

        let lexer = Lexer::lex("1234_5678 ");
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 1);
        assert_eq!(lexer.tokens[0].text, "1234_5678");
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 8 });

        let lexer = Lexer::lex("  123'sh111bbb ");
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 1);
        assert_eq!(lexer.tokens[0].text, "123'sh111bbb");
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 2 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 13 });

        let lexer = Lexer::lex("1.0 1.0e+30");
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 2);
        assert_eq!(lexer.tokens[0].text, "1.0");
        assert_eq!(lexer.tokens[1].text, "1.0e+30");
    }

    #[test]
    fn operator() {
        let lexer = Lexer::lex("+~|<<<^~-");
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 5);
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[1].span.from, Location { row: 0, col: 1 });
        assert_eq!(lexer.tokens[1].span.to, Location { row: 0, col: 2 });
        assert_eq!(lexer.tokens[2].span.from, Location { row: 0, col: 3 });
        assert_eq!(lexer.tokens[2].span.to, Location { row: 0, col: 5 });
    }

    #[test]
    fn string() {
        let lexer = Lexer::lex(r#""abcde\t\n\r\\\"\"""#);
        println!("{:?}", lexer.tokens);
        println!("{:?}", lexer.diag);
        assert_eq!(lexer.tokens.len(), 1);
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 18 });
        assert_eq!(lexer.diag.len(), 1); // \r
        assert_eq!(lexer.diag[0].msg, Message::UnrecognizedEscapeCharacter('r'));
        // \r
    }

    #[test]
    fn identifier() {
        let lexer = Lexer::lex(r#""abc"abc"#);
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 2);
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 4 });
        assert_eq!(lexer.tokens[1].span.from, Location { row: 0, col: 5 });
        assert_eq!(lexer.tokens[1].span.to, Location { row: 0, col: 7 });

        let lexer = Lexer::lex(r#"abc "abc""#);
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 2);
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 2 });
        assert_eq!(lexer.tokens[1].span.from, Location { row: 0, col: 4 });
        assert_eq!(lexer.tokens[1].span.to, Location { row: 0, col: 8 });
    }

    #[test]
    fn keyword() {
        let lexer = Lexer::lex(r#"and andd an always"#);
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 4);
        assert_eq!(lexer.tokens[0].span.from, Location { row: 0, col: 0 });
        assert_eq!(lexer.tokens[0].span.to, Location { row: 0, col: 2 });
        assert_eq!(lexer.tokens[0].token, Token::And);
        assert_eq!(lexer.tokens[1].span.from, Location { row: 0, col: 4 });
        assert_eq!(lexer.tokens[1].span.to, Location { row: 0, col: 7 });
        assert_eq!(lexer.tokens[1].token, Token::Identifier);
        assert_eq!(lexer.tokens[2].span.from, Location { row: 0, col: 9 });
        assert_eq!(lexer.tokens[2].span.to, Location { row: 0, col: 10 });
        assert_eq!(lexer.tokens[2].token, Token::Identifier);
        assert_eq!(lexer.tokens[3].span.from, Location { row: 0, col: 12 });
        assert_eq!(lexer.tokens[3].span.to, Location { row: 0, col: 17 });
        assert_eq!(lexer.tokens[3].token, Token::Always);
    }
}
