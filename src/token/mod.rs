use crate::token::Keyword::*;
use crate::token::Literal::*;
use crate::token::Spec::*;
use regex::Regex;

const SPEC: &str = "=+-!*/<>,;(){}";

const SPEC_PATTERNS: [&str; 2] = ["==", "!="];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Spec {
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Equal,
    NotEqual,
}

const TOKEN_REGEXP: [&str; 2] = ["^[A-Za-z]\\w*", "^\\d+"];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Literal {
    Ident,
    Int,
}

const KEYWORDS: [&str; 7] = ["fn", "let", "true", "false", "if", "else", "return"];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Keyword {
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    Spec(Spec),
    Literal(Literal),
    Keyword(Keyword),
    Illegal,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub const fn new(token_type: TokenType, literal: Option<String>, line: usize) -> Token {
        Token {
            token_type,
            literal,
            line,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TokenPos {
    pub token_type: TokenType,
    pub end: usize,
}

impl TokenPos {
    pub fn new(token_type: TokenType, end: usize) -> TokenPos {
        TokenPos { token_type, end }
    }

    pub fn token(&self, literal: Option<&str>, line: usize) -> Token {
        let token_type = self.token_type;
        Token {
            token_type,
            literal: literal.map(|s| s.to_string()),
            line,
        }
    }
}

impl TokenType {
    pub fn match_spec(input: &str, start: usize, ch: char) -> Option<TokenPos> {
        if let Some(i) = SPEC.find(ch) {
            for (j, p) in SPEC_PATTERNS.iter().enumerate() {
                if input[start..].starts_with(p) {
                    return Spec::from_int(SPEC.len() + j)
                        .map(|it| TokenPos::new(TokenType::Spec(it), start + p.len()));
                }
            }
            return Spec::from_int(i)
                .map(|it| TokenPos::new(TokenType::Spec(it), start + ch.len_utf8()));
        }
        None
    }

    pub fn literal_token_matcher() -> Box<dyn Fn(&str, usize) -> Option<TokenPos>> {
        let regexps: Vec<Regex> = TOKEN_REGEXP
            .iter()
            .map(|pattern| Regex::new(pattern).unwrap())
            .collect();
        let keywords: Vec<String> = KEYWORDS.iter().map(|s| s.to_string()).collect();
        Box::new(move |input: &str, start: usize| {
            for (i, r) in regexps.iter().enumerate() {
                if let Some(m) = r.find(&input[start..]) {
                    for (j, kw) in keywords.iter().enumerate() {
                        let end = start + m.end();
                        if kw.eq(&input[start..end]) {
                            return Keyword::from_int(j)
                                .map(|it| TokenPos::new(TokenType::Keyword(it), end));
                        }
                    }
                    return Literal::from_int(i)
                        .map(|it| TokenPos::new(TokenType::Literal(it), start + m.end()));
                }
            }
            None
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}

impl Spec {
    fn from_int(i: usize) -> Option<Spec> {
        match i {
            0 => Some(Assign),
            1 => Some(Plus),
            2 => Some(Minus),
            3 => Some(Bang),
            4 => Some(Asterisk),
            5 => Some(Slash),
            6 => Some(Lt),
            7 => Some(Gt),
            8 => Some(Comma),
            9 => Some(Semicolon),
            10 => Some(Lparen),
            11 => Some(Rparen),
            12 => Some(Lbrace),
            13 => Some(Rbrace),
            14 => Some(Equal),
            15 => Some(NotEqual),
            _ => None,
        }
    }
}

impl Literal {
    fn from_int(i: usize) -> Option<Literal> {
        match i {
            0 => Some(Ident),
            1 => Some(Int),
            _ => None,
        }
    }
}

impl Keyword {
    fn from_int(i: usize) -> Option<Keyword> {
        match i {
            0 => Some(Function),
            1 => Some(Let),
            2 => Some(True),
            3 => Some(False),
            4 => Some(If),
            5 => Some(Else),
            6 => Some(Return),
            _ => None,
        }
    }
}
