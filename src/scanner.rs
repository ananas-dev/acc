#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Star,
    Slash,
    Semicolon,
    Dot,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Arrow,
    String(String),
    Number(String),
    Character(String),
    Identifier(String),
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

pub struct Scanner<'a> {
    stream: &'a [char],
    file_path: String,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(stream: &'a [char], file_path: String) -> Scanner<'a> {
        Scanner {
            stream,
            file_path,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_one(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.new_token(TokenKind::Eof);
        }

        let c = self.advance();

        if c.is_ascii_alphabetic() {
            return self.identifier(c)
        }

        if c.is_ascii_digit() {
            return self.number(c);
        }

        match c {
            '(' => self.new_token(TokenKind::LeftParen),
            ')' => self.new_token(TokenKind::RightParen),
            '{' => self.new_token(TokenKind::LeftBrace),
            '}' => self.new_token(TokenKind::RightBrace),
            '+' => {
                if self.match_char('+') {
                    self.new_token(TokenKind::PlusPlus)
                } else {
                    self.new_token(TokenKind::Plus)
                }
            },
            '-' => {
                if self.match_char('-') {
                    self.new_token(TokenKind::MinusMinus)
                } else if self.match_char('>') {
                    self.new_token(TokenKind::Arrow)
                } else {
                    self.new_token(TokenKind::Minus)
                }
            },
            '*' => self.new_token(TokenKind::Star),
            '/' => self.new_token(TokenKind::Slash),
            '.' => self.new_token(TokenKind::Dot),
            ';' => self.new_token(TokenKind::Semicolon),
            '!' => {
                if self.match_char('=') {
                    self.new_token(TokenKind::BangEqual)
                } else {
                    self.new_token(TokenKind::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.new_token(TokenKind::EqualEqual)
                } else {
                    self.new_token(TokenKind::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.new_token(TokenKind::LessEqual)
                } else {
                    self.new_token(TokenKind::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.new_token(TokenKind::GreaterEqual)
                } else {
                    self.new_token(TokenKind::Greater)
                }
            }
            '"' => self.string(),
            _ => panic!("Unknow char: {}", c),
        }
    }

    fn identifier(&mut self, first: char) -> Token {
        let mut res = String::new();
        res.push(first);

        while self.peek().is_ascii_alphanumeric() {
            res.push(self.peek());
            self.advance();
        }

        self.new_token(TokenKind::Identifier(res))
    }

    fn number(&mut self, first: char) -> Token {
        let mut res = String::new();
        res.push(first);

        while self.peek().is_ascii_digit() {
            res.push(self.peek());
            self.advance();
        }

        self.new_token(TokenKind::Number(res))
    }

    fn string(&mut self) -> Token {
        let mut res = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            if self.peek() == '\\' {
                match self.peek_next() {
                    'a' => res.push('\x07'),
                    'b' => res.push('\x08'),
                    'e' => res.push('\x1b'),
                    'f' => res.push('\x0c'),
                    'n' => res.push('\n'),
                    'r' => res.push('\r'),
                    't' => res.push('\t'),
                    'v' => res.push('\x0b'),
                    '\\' => res.push('\\'),
                    '\'' => res.push('\''),
                    '"' => res.push('"'),
                    '?' => res.push('\x3f'),
                    _ => panic!(),
                }
                self.advance();
                self.advance();
            } else {
                res.push(self.peek());
                self.advance();
            }
        }

        if self.is_at_end() {
            panic!()
        }

        self.advance();

        self.new_token(TokenKind::String(res))
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else if self.peek_next() == '*' {
                        self.advance();
                        self.advance();

                        while !self.is_at_end() {
                            if self.peek() == '*' && self.peek_next() == '/' {
                                self.advance();
                                self.advance();
                                break;
                            }
                            self.advance();
                        }

                        self.advance();
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            };
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.stream[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.stream[self.current - 1]
    }

    // FIXME: if branch can be avoided
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.stream[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.stream.len() {
            '\0'
        } else {
            self.stream[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.stream.len()
    }

    fn new_token(&mut self, kind: TokenKind) -> Token {
        Token {
            kind,
            line: self.line,
        }
    }

}
