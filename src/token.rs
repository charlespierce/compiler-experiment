#[derive(Debug)]
pub enum Token {
    OpenParenthesis,
    CloseParenthesis,
    Number(String),
    Identifier(String),
    String(String),
}

pub struct Tokenizer {
    src: String,
    current_pos: usize,
    cursor: usize,
    len: usize,
    ch: Option<char>,
}

impl Tokenizer {
    pub fn new(src: String) -> Self {
        let len = src.len();
        Tokenizer {
            src,
            current_pos: 0,
            cursor: 0,
            len,
            ch: Some('\n'),
        }
    }

    fn bump(&mut self) {
        self.current_pos = self.cursor;
        if self.cursor < self.len {
            let ch = self.src[self.cursor..].chars().next().unwrap();
            self.ch = Some(ch);
            self.cursor += ch.len_utf8();
        } else {
            self.ch = None;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.map_or(false, |ch| ch.is_whitespace()) {
            self.bump();
        }
    }

    fn eof(&self) -> bool {
        self.ch.is_none()
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.ch.map(|ch| match ch {
            '(' => {
                self.bump();
                Token::OpenParenthesis
            }
            ')' => {
                self.bump();
                Token::CloseParenthesis
            }
            '"' => {
                self.bump();
                let start_pos = self.current_pos;

                while self.ch != Some('"') {
                    if self.eof() {
                        panic!("Unclosed string found!");
                    }
                    self.bump();
                }

                let res = self.src[start_pos..self.current_pos].to_string();
                self.bump();

                Token::String(res)
            }
            n if n.is_numeric() => {
                let start_pos = self.current_pos;
                self.bump();

                while self.ch.map_or(false, |c| c.is_numeric()) {
                    self.bump();
                }

                Token::Number(self.src[start_pos..self.current_pos].to_string())
            }
            a if a.is_alphabetic() => {
                let start_pos = self.current_pos;
                self.bump();

                while self.ch.map_or(false, |c| c.is_alphabetic()) {
                    self.bump();
                }

                Token::Identifier(self.src[start_pos..self.current_pos].to_string())
            }
            _ => {
                panic!("Unknown Character Found!");
            }
        })
    }
}
