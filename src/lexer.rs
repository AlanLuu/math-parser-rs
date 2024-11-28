use std::num::IntErrorKind;

use crate::{intorfloat::IntOrFloat, token::Token};

pub struct Lexer {
    string: String,
    str_len: usize,
    pos: usize,
    line_num: i32,
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

impl Lexer {
    pub fn new(str: &str) -> Self {
        let string = String::from(str);
        let str_len = string.chars().count();
        Lexer{
            string,
            str_len,
            pos: 0,
            line_num: 1,
        }
    }

    fn error(&self, err_str: &str) -> String {
        format!(
            "{}\n[Line {}]",
            err_str,
            self.line_num,
        )
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.str_len
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            let character = self.string[self.pos..self.pos+1]
                .chars()
                .nth(0)
                .unwrap();
            if is_digit(character) {
                let start_pos = self.pos;
                self.pos += 1;
                let num = if !self.is_at_end() {
                    let mut end_character = self.string[start_pos+1..self.pos+1]
                        .chars()
                        .nth(0)
                        .unwrap();
                    let mut is_decimal = false;
                    while !self.is_at_end() && (is_digit(end_character) || end_character == '.') {
                        self.pos += 1;
                        if self.is_at_end() {
                            break;
                        }
                        if end_character == '.' {
                            is_decimal = true;
                            self.pos += 1;
                            if self.is_at_end() {
                                break;
                            }
                        }
                        end_character = self.string[self.pos..self.pos+1]
                            .chars()
                            .nth(0)
                            .unwrap();
                    }
                    if is_decimal {
                        let parse_str = &self.string[start_pos..self.pos];
                        let parse_result = parse_str.parse::<f64>();
                        match parse_result {
                            Ok(num) => IntOrFloat::Float(num),
                            Err(_) => {
                                let err_str = format!(
                                    "Failed to parse float literal '{}'.",
                                    parse_str,
                                );
                                return Err(self.error(&err_str));
                            }
                        }
                    } else {
                        let parse_str = &self.string[start_pos..self.pos];
                        let parse_result = parse_str
                            .parse::<i32>();
                        match parse_result {
                            Ok(num) => IntOrFloat::Int(num),
                            Err(err) => {
                                match err.kind() {
                                    IntErrorKind::PosOverflow => {
                                        let err_str = format!(
                                            "Integer literal '{}' is too large.",
                                            parse_str,
                                        );
                                        return Err(self.error(&err_str));
                                    }
                                    _ => {
                                        let err_str = format!(
                                            "Failed to parse integer literal '{}'.",
                                            parse_str,
                                        );
                                        return Err(self.error(&err_str));
                                    },
                                }
                            },
                        }
                    }
                } else {
                    IntOrFloat::Int(character.to_digit(10).unwrap() as i32)
                };
                tokens.push(Token::Number { num });
            } else {
                match character {
                    '+' => tokens.push(Token::Plus),
                    '-' => tokens.push(Token::Minus),
                    '*' => tokens.push(Token::Star),
                    '/' => tokens.push(Token::Slash),
                    '(' => tokens.push(Token::LeftParen),
                    ')' => tokens.push(Token::RightParen),
                    '\n' => self.line_num += 1,
                    ' ' | '\r' | '\t' => (),
                    _ => {
                        let err_str = format!(
                            "Unexpected character '{}'.",
                            character,
                        );
                        return Err(self.error(&err_str));
                    }
                }
                self.pos += 1;
            }
        }
        Ok(tokens)
    }
}
