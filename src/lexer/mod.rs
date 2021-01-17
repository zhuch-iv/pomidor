use crate::token::{Token, TokenPos, TokenType};

pub struct Lexer {
    literal_matcher: Box<dyn Fn(&str, usize) -> Option<TokenPos>>,
}

pub struct LexerIterator<'l, 'i> {
    pos: usize,
    input: &'i str,
    current_line: usize,
    lexer: &'l Lexer,
}

impl<'l, 'i> Lexer {
    pub fn new() -> Lexer {
        Lexer {
            literal_matcher: TokenType::literal_token_matcher(),
        }
    }

    pub fn tokenize(&'l self, input: &'i str) -> LexerIterator<'l, 'i> {
        LexerIterator {
            pos: 0,
            input,
            current_line: 0,
            lexer: self,
        }
    }

    pub fn match_token(&self, input: &'i str, start: usize) -> Option<TokenPos> {
        (self.literal_matcher)(input, start)
    }
}

impl<'l, 'a> LexerIterator<'l, 'a> {
    fn skip_whitespaces(&mut self) -> Option<char> {
        self.input[self.pos..].chars().find(|ch| {
            if ch.is_whitespace() {
                if ch.eq(&'\n') {
                    self.current_line += 1;
                }
                self.pos += ch.len_utf8();
                false
            } else {
                true
            }
        })
    }

    fn produce(&mut self, pos: TokenPos) -> Token {
        let start = self.pos;
        self.pos = pos.end;
        match pos.token_type {
            TokenType::Literal(_) => {
                pos.token(Some(&self.input[start..pos.end]), self.current_line)
            }
            _ => pos.token(None, self.current_line),
        }
    }

    fn illegal_or_none(&mut self) -> Option<Token> {
        if self.pos < self.input.len() {
            self.pos = self.input.len();
            return Some(Token {
                token_type: TokenType::Illegal,
                literal: None,
                line: self.current_line,
            });
        }
        None
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces()
            .and_then(|ch| TokenType::match_spec(self.input, self.pos, ch))
            .or_else(|| self.lexer.match_token(self.input, self.pos))
            .map(|m| self.produce(m))
            .or_else(|| self.illegal_or_none())
    }
}

impl<'l, 'a> Iterator for LexerIterator<'l, 'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::Keyword::*;
    use crate::token::Literal::*;
    use crate::token::Spec::*;
    use crate::token::TokenType::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_illegal() {
        let code = "-66 бррр";
        let lexer = Lexer::new();
        lexer.tokenize(code).enumerate().for_each(|(i, token)| {
            TEST_ILLEGAL[i].assert_eq(token);
        });
    }

    #[test]
    fn test1() {
        let code = "=+(){},;";
        let lexer = Lexer::new();
        lexer.tokenize(code).enumerate().for_each(|(i, token)| {
            TEST_1[i].assert_eq(token);
        });
    }

    #[test]
    fn test2() {
        let code = "let x = 5 + 5;";
        let lexer = Lexer::new();
        lexer.tokenize(code).enumerate().for_each(|(i, token)| {
            TEST_2[i].assert_eq(token);
        });
    }

    #[test]
    fn test3() {
        let code = "let five = 5; \n\
            let ten = 10; \n\
            \n\
            let add = fn(x, y) { \n\
                x + y; \n\
            }; \n\
            let result = add(five, ten);";
        let lexer = Lexer::new();
        lexer.tokenize(code).enumerate().for_each(|(i, token)| {
            TEST_3[i].assert_eq(token);
        });
    }

    #[test]
    fn test4() {
        let code = "let five = 5; \n\
            let ten = 10; \n\
            let add = fn(x, y) { \n\
                x + y; \n\
            }; \n\
            let result = add(five, ten); \n\
            !-/*5; \n\
            5 < 10 > 5; \n\
            if (5 == 10) { \n\
                return true; \n\
            } else { \n\
                return false; \n\
            }";
        let lexer = Lexer::new();
        lexer.tokenize(code).enumerate().for_each(|(i, token)| {
            TEST_4[i].assert_eq(token);
        });
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct TestToken {
        token_type: TokenType,
        literal: Option<&'static str>,
        line: usize,
    }

    impl TestToken {
        pub const fn new(
            token_type: TokenType,
            literal: Option<&'static str>,
            line: usize,
        ) -> TestToken {
            TestToken {
                token_type,
                literal,
                line,
            }
        }

        pub fn assert_eq(&self, token: Token) {
            assert_eq!(self.token_type, token.token_type);
            assert_eq!(self.literal.map(|s| s.to_owned()), token.literal);
            assert_eq!(self.line, token.line);
        }
    }

    const TEST_ILLEGAL: [TestToken; 3] = [
        TestToken::new(Spec(Minus), None, 0),
        TestToken::new(Literal(Int), Some("66"), 0),
        TestToken::new(Illegal, None, 0),
    ];

    const TEST_1: [TestToken; 8] = [
        TestToken::new(Spec(Assign), None, 0),
        TestToken::new(Spec(Plus), None, 0),
        TestToken::new(Spec(Lparen), None, 0),
        TestToken::new(Spec(Rparen), None, 0),
        TestToken::new(Spec(Lbrace), None, 0),
        TestToken::new(Spec(Rbrace), None, 0),
        TestToken::new(Spec(Comma), None, 0),
        TestToken::new(Spec(Semicolon), None, 0),
    ];

    const TEST_2: [TestToken; 7] = [
        TestToken::new(Keyword(Let), None, 0),
        TestToken::new(Literal(Ident), Some("x"), 0),
        TestToken::new(Spec(Assign), None, 0),
        TestToken::new(Literal(Int), Some("5"), 0),
        TestToken::new(Spec(Plus), None, 0),
        TestToken::new(Literal(Int), Some("5"), 0),
        TestToken::new(Spec(Semicolon), None, 0),
    ];

    const TEST_3: [TestToken; 36] = [
        TestToken::new(Keyword(Let), None, 0),
        TestToken::new(Literal(Ident), Some("five"), 0),
        TestToken::new(Spec(Assign), None, 0),
        TestToken::new(Literal(Int), Some("5"), 0),
        TestToken::new(Spec(Semicolon), None, 0),
        TestToken::new(Keyword(Let), None, 1),
        TestToken::new(Literal(Ident), Some("ten"), 1),
        TestToken::new(Spec(Assign), None, 1),
        TestToken::new(Literal(Int), Some("10"), 1),
        TestToken::new(Spec(Semicolon), None, 1),
        TestToken::new(Keyword(Let), None, 3),
        TestToken::new(Literal(Ident), Some("add"), 3),
        TestToken::new(Spec(Assign), None, 3),
        TestToken::new(Keyword(Function), None, 3),
        TestToken::new(Spec(Lparen), None, 3),
        TestToken::new(Literal(Ident), Some("x"), 3),
        TestToken::new(Spec(Comma), None, 3),
        TestToken::new(Literal(Ident), Some("y"), 3),
        TestToken::new(Spec(Rparen), None, 3),
        TestToken::new(Spec(Lbrace), None, 3),
        TestToken::new(Literal(Ident), Some("x"), 4),
        TestToken::new(Spec(Plus), None, 4),
        TestToken::new(Literal(Ident), Some("y"), 4),
        TestToken::new(Spec(Semicolon), None, 4),
        TestToken::new(Spec(Rbrace), None, 5),
        TestToken::new(Spec(Semicolon), None, 5),
        TestToken::new(Keyword(Let), None, 6),
        TestToken::new(Literal(Ident), Some("result"), 6),
        TestToken::new(Spec(Assign), None, 6),
        TestToken::new(Literal(Ident), Some("add"), 6),
        TestToken::new(Spec(Lparen), None, 6),
        TestToken::new(Literal(Ident), Some("five"), 6),
        TestToken::new(Spec(Comma), None, 6),
        TestToken::new(Literal(Ident), Some("ten"), 6),
        TestToken::new(Spec(Rparen), None, 6),
        TestToken::new(Spec(Semicolon), None, 6),
    ];

    const TEST_4: [TestToken; 65] = [
        TestToken::new(Keyword(Let), None, 0),
        TestToken::new(Literal(Ident), Some("five"), 0),
        TestToken::new(Spec(Assign), None, 0),
        TestToken::new(Literal(Int), Some("5"), 0),
        TestToken::new(Spec(Semicolon), None, 0),
        TestToken::new(Keyword(Let), None, 1),
        TestToken::new(Literal(Ident), Some("ten"), 1),
        TestToken::new(Spec(Assign), None, 1),
        TestToken::new(Literal(Int), Some("10"), 1),
        TestToken::new(Spec(Semicolon), None, 1),
        TestToken::new(Keyword(Let), None, 2),
        TestToken::new(Literal(Ident), Some("add"), 2),
        TestToken::new(Spec(Assign), None, 2),
        TestToken::new(Keyword(Function), None, 2),
        TestToken::new(Spec(Lparen), None, 2),
        TestToken::new(Literal(Ident), Some("x"), 2),
        TestToken::new(Spec(Comma), None, 2),
        TestToken::new(Literal(Ident), Some("y"), 2),
        TestToken::new(Spec(Rparen), None, 2),
        TestToken::new(Spec(Lbrace), None, 2),
        TestToken::new(Literal(Ident), Some("x"), 3),
        TestToken::new(Spec(Plus), None, 3),
        TestToken::new(Literal(Ident), Some("y"), 3),
        TestToken::new(Spec(Semicolon), None, 3),
        TestToken::new(Spec(Rbrace), None, 4),
        TestToken::new(Spec(Semicolon), None, 4),
        TestToken::new(Keyword(Let), None, 5),
        TestToken::new(Literal(Ident), Some("result"), 5),
        TestToken::new(Spec(Assign), None, 5),
        TestToken::new(Literal(Ident), Some("add"), 5),
        TestToken::new(Spec(Lparen), None, 5),
        TestToken::new(Literal(Ident), Some("five"), 5),
        TestToken::new(Spec(Comma), None, 5),
        TestToken::new(Literal(Ident), Some("ten"), 5),
        TestToken::new(Spec(Rparen), None, 5),
        TestToken::new(Spec(Semicolon), None, 5),
        TestToken::new(Spec(Bang), None, 6),
        TestToken::new(Spec(Minus), None, 6),
        TestToken::new(Spec(Slash), None, 6),
        TestToken::new(Spec(Asterisk), None, 6),
        TestToken::new(Literal(Int), Some("5"), 6),
        TestToken::new(Spec(Semicolon), None, 6),
        TestToken::new(Literal(Int), Some("5"), 7),
        TestToken::new(Spec(Lt), None, 7),
        TestToken::new(Literal(Int), Some("10"), 7),
        TestToken::new(Spec(Gt), None, 7),
        TestToken::new(Literal(Int), Some("5"), 7),
        TestToken::new(Spec(Semicolon), None, 7),
        TestToken::new(Keyword(If), None, 8),
        TestToken::new(Spec(Lparen), None, 8),
        TestToken::new(Literal(Int), Some("5"), 8),
        TestToken::new(Spec(Equal), None, 8),
        TestToken::new(Literal(Int), Some("10"), 8),
        TestToken::new(Spec(Rparen), None, 8),
        TestToken::new(Spec(Lbrace), None, 8),
        TestToken::new(Keyword(Return), None, 9),
        TestToken::new(Keyword(True), None, 9),
        TestToken::new(Spec(Semicolon), None, 9),
        TestToken::new(Spec(Rbrace), None, 10),
        TestToken::new(Keyword(Else), None, 10),
        TestToken::new(Spec(Lbrace), None, 10),
        TestToken::new(Keyword(Return), None, 11),
        TestToken::new(Keyword(False), None, 11),
        TestToken::new(Spec(Semicolon), None, 11),
        TestToken::new(Spec(Rbrace), None, 12),
    ];
}
