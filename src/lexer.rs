use crate::diagnostic::Diagnostic;
use strcursor::StrCursor;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Span {
    pub from: Location,
    pub to: Location,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    // Types
    Number,
    Identifier,
    String,
    CompilerDirective,
    Directive,
    Comment,

    // Keywords, Annex B
    Always,
    AlwaysComb,
    AlwaysFf,
    And,
    Assign,
    Automatic,
    Begin,
    Buf,
    BufIf0,
    BufIf1,
    Case,
    Casex,
    Casez,
    Cell,
    Cmos,
    Config,
    Deassign,
    Default,
    DefParam,
    Design,
    Disable,
    Edge,
    Else,
    End,
    EndCase,
    EndConfig,
    EndFunction,
    EndGenerate,
    EndModule,
    EndPrimitive,
    EndSpecify,
    EndTable,
    EndTask,
    Event,
    For,
    Force,
    Forever,
    Fork,
    Function,
    Generate,
    GenVar,
    HighZ0,
    HighZ1,
    If,
    IfNone,
    IncDir,
    Include,
    Initial,
    InOut,
    Input,
    Instance,
    Integer,
    Join,
    Large,
    LibList,
    Library,
    Localparam,
    MacroModule,
    Medium,
    Module,
    Nand,
    NegEdge,
    Nmos,
    Nor,
    NoShowCancelled,
    Not,
    NotIf0,
    NotIf1,
    Or,
    Output,
    Parameter,
    Pmos,
    PosEdge,
    Primitive,
    Pull0,
    Pull1,
    PullDown,
    PullUp,
    PulseStyleOnEvent,
    PulseStyleOnDetect,
    Rcmos,
    Real,
    Realtime,
    Reg,
    Release,
    Repeat,
    Rnmos,
    Rpmos,
    Rtran,
    RtranIf0,
    RtranIf1,
    Scalared,
    ShowCancelled,
    Signed,
    Small,
    Specify,
    Specparam,
    Strong0,
    Strong1,
    Supply0,
    Supply1,
    Table,
    Task,
    Time,
    Tran,
    TranIf0,
    TranIf1,
    Tri,
    Tri0,
    Tri1,
    TriAnd,
    TriOr,
    TriReg,
    Unsigned,
    Use,
    Vectored,
    Wait,
    Wand,
    Weak0,
    Weak1,
    Wire,
    Wor,
    Xnor,
    Xor,

    // Delimiter
    Sharp,
    LParen, // ()
    RParen,
    LBracket, // []
    RBracket,
    LBraces, // {}
    RBraces,
    Colon,
    Comma,
    Semicolon,
    Dot,

    // Operators
    OpEqual,
    OpAt,
    OpDivide,
    OpMinus,
    OpNot,
    OpPlus,
    OpInvert,
    OpMultiply,
    OpChoice,
    OpEqualTo,
    OpAssign,
    OpLessThan,
    OpGreaterThan,
    OpLeftShift,
    OpGreaterEqual,
    OpAnd,

    None,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ParsedToken<'a> {
    pub span: Span,
    pub token: Token,
    pub text: &'a str,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    cursor: StrCursor<'a>,
    loc: Location,
    tokens: Vec<ParsedToken<'a>>,
    diag: Vec<Diagnostic>,
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

    fn diag(&mut self, from: Location, to: Location, msg: String) {
        self.diag.push(Diagnostic {
            pos: Span { from, to },
            message: msg,
        });
    }

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
                                self.cursor = cursor;
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
                        self.diag(from, self.loc, format!("Multiline comment not closed"));
                        self.cursor = cursor;
                        return true;
                    }
                }
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
                '/' => {
                    if self.comment() {
                        continue;
                    }
                }
                _ => {
                    self.diag(self.loc, self.loc, format!("Unexpected char: {}", gc));
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
}
