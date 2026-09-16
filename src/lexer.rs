use std::{
    iter::Peekable,
    str::CharIndices,
};
use crate::error::LexafError;
use crate::tokens::{
    Span,
    Token,
    SpannedToken,
    StrIntr,
};

/// A lexical analyzer that converts a raw string input into a sequence of tokens.
pub struct Lexer<'a> {
    chars: Peekable<CharIndices<'a>>,
    input_len: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` instance from the given string input.
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(), 
            input_len: input.len(),
        }
    }

    /// Helper to get the current byte index (or the end of the string if EOF)
    fn pos(&mut self) -> usize {
        self.chars.peek().map(|&(idx, _)| idx).unwrap_or(self.input_len)
    }

    /// Checks if a given string slice consists entirely of base-10 digits (integer).
    fn is_number(str: &str) -> bool {
        let s = str.strip_prefix('-').unwrap_or(str);
        !s.is_empty() && s.chars().all(|c| c.is_digit(10))
    }

    /// Checks if a given string slice represents a floating-point (decimal) number.
    fn is_decimal(str: &str) -> bool {
        let s = str.strip_prefix('-').unwrap_or(str);
        let mut dot_count = 0;
        let mut has_digits = false;
        for c in s.chars() {
            if c.is_digit(10) {
                has_digits = true;
            } else if c == '.' {
                dot_count += 1;
            } else {
                return false; 
            }
        }
        has_digits && dot_count == 1
    }

    /// Processes the input character stream and builds a vector of spanned tokens.
    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken>, LexafError> {
        let mut tokens = Vec::new();
        
        while let Some(&(start, c)) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                }
                '\n' => {
                    self.chars.next();
                    tokens.push(SpannedToken {
                        token: Token::NewLine,
                        span: Span { start, end: start + 1 },
                    });
                }
                '"' => {
                    self.chars.next(); 
                    let mut str_tokens = Vec::new();
                    let mut lit = String::new();
                    loop {
                        match self.chars.next() {
                            Some((_, '"')) => break,
                            Some((_, '{')) => {
                                if !lit.is_empty() {
                                    str_tokens.push(StrIntr::Literal(lit.clone()));
                                    lit.clear(); 
                                }
                                let mut var = String::new();
                                loop {
                                    match self.chars.next() {
                                        Some((_, '}')) => break,
                                        Some((_, '"')) => {
                                            return Err(LexafError::UnclosedDelimiter {
                                                delimiter: "}".to_string(),
                                                span: (start..self.pos()).into(),
                                            });
                                        }
                                        Some((_, v)) => var.push(v),
                                        None => {
                                            return Err(LexafError::UnclosedDelimiter {
                                                delimiter: "}".to_string(),
                                                span: (start..self.pos()).into(),
                                            });
                                        }
                                    }
                                }
                                str_tokens.push(StrIntr::Variable(var));
                            }
                            Some((_, ch)) => lit.push(ch),
                            None => {
                                return Err(LexafError::UnclosedDelimiter {
                                    delimiter: "\"".to_string(),
                                    span: (start..self.pos()).into(),
                                });
                            }
                        }
                    }
                    if !lit.is_empty() {
                        str_tokens.push(StrIntr::Literal(lit));
                    }
                    let end = self.pos();
                    tokens.push(SpannedToken {
                        token: Token::Str(str_tokens),
                        span: Span { start, end },
                    });
                }
                ';' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::SemiCln, span: Span { start, end: start + 1 } });
                }
                ',' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::Comma, span: Span { start, end: start + 1 } });
                }
                '{' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::LBrc, span: Span { start, end: start + 1 } });
                }
                '}' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::RBrc, span: Span { start, end: start + 1 } });
                }
                '[' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::LSqr, span: Span { start, end: start + 1 } });
                }
                ']' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::RSqr, span: Span { start, end: start + 1 } });
                }
                '<' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::RdrctIn, span: Span { start, end: start + 1 } });
                }
                '&' => {
                    self.chars.next();
                    if let Some(&(_, '&')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(SpannedToken { token: Token::AndAnd, span: Span { start, end: start + 2 } });
                    } else {
                        tokens.push(SpannedToken { token: Token::And, span: Span { start, end: start + 1 } });
                    }
                }
                '!' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(SpannedToken { token: Token::NotEq, span: Span { start, end: start + 2 } });
                    } else {
                        tokens.push(SpannedToken { token: Token::Bang, span: Span { start, end: start + 1 } });
                    }
                }
                '=' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(SpannedToken { token: Token::EqEq, span: Span { start, end: start + 2 } });
                    } else {
                        tokens.push(SpannedToken { token: Token::Assign, span: Span { start, end: start + 1 } });
                    }
                }
                '|' => {
                    self.chars.next();
                    if let Some(&(_, '|')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(SpannedToken { token: Token::OrOr, span: Span { start, end: start + 2 } });
                    } else {
                        tokens.push(SpannedToken { token: Token::Pipe, span: Span { start, end: start + 1 } });
                    }
                }
                '>' => {
                    self.chars.next();
                    if let Some(&(_, '>')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(SpannedToken { token: Token::Append, span: Span { start, end: start + 2 } });
                    } else {
                        tokens.push(SpannedToken { token: Token::RdrctOut, span: Span { start, end: start + 1 } });
                    }
                }
                '#' => {
                    self.chars.next();
                    while let Some(&(_, ch)) = self.chars.peek() {
                        if ch == '\n' {
                            break;
                        } else {
                            self.chars.next();
                        }
                    }
                }
                '$' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::Eval, span: Span { start, end: start + 1 } });
                    
                    while let Some(&(inner_start, ch)) = self.chars.peek() {
                        match ch { 
                            ' ' | '\n' | '\t' | '\r' => {
                                self.chars.next();
                            } 
                            '{' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::LBrc, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            '}' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::RBrc, span: Span { start: inner_start, end: inner_start + 1 } });
                                break;
                            }
                            '(' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::LPths, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            ')' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::RPths, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            '+' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::Plus, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            '-' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::Minus, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            '*' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::Multiply, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            '/' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::Divide, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            '%' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::Modulo, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            '^' => {
                                self.chars.next();
                                tokens.push(SpannedToken { token: Token::Power, span: Span { start: inner_start, end: inner_start + 1 } });
                            }
                            _ => {
                                let mut word = String::new();
                                while let Some(&(_, mw)) = self.chars.peek() {
                                    match mw {
                                          ' ' | '\n' | '\t'| '\r'
                                        | '+' | '-'  | '*' | '{' | '(' 
                                        | '/' | '%'  | '^' | '}' | ')' => {
                                            break;
                                        }
                                        _ => {
                                            word.push(mw);
                                            self.chars.next();
                                        }                                      
                                    }
                                }
                                
                                let inner_end = self.pos();
                                let token = if Self::is_decimal(&word) {
                                    if let Ok(num) = word.parse::<f64>() {
                                        Token::Float(num)
                                    } else {
                                        Token::Word(word)
                                    }
                                } else if Self::is_number(&word) {
                                    if let Ok(num) = word.parse::<i64>() {
                                        Token::Num(num)
                                    } else {
                                        Token::Word(word)
                                    }
                                } else {
                                    Token::Word(word)
                                };
                                
                                tokens.push(SpannedToken { token, span: Span { start: inner_start, end: inner_end } });
                            }
                        }
                    } 
                }
                _ => {
                    let mut word = String::new();
                    while let Some(&(_, ch)) = self.chars.peek() {
                        match ch {
                              ' ' | '\n' | '\t' | '\r' | '"' | '='
                            | ';' | ','  | '&'  | '|'  | '!' | '#'
                            | '>' | '{'  | '}'  | '['  | ']' | '$' | '<' => { 
                                break;
                            }
                            '\\' => {
                                self.chars.next();
                                if let Some(&(_, ch)) = self.chars.peek() {
                                    word.push(ch);
                                    self.chars.next();
                                }
                            }
                            _ => {
                                word.push(ch);
                                self.chars.next();
                            }
                        }
                    }
                    
                    let end = self.pos();
                    let token = match word.as_str() {
                        "let" => Token::Let,
                        "print" => Token::Print,
                        "if" => Token::If,
                        "elif" => Token::Elif,
                        "else" => Token::Else,
                        "for" => Token::For,
                        "while" => Token::While,
                        "in" => Token::In,
                        "to" => Token::To,
                        "break" => Token::Break,
                        "true" => Token::True,
                        "false" => Token::False,
                        _ => {
                            if Self::is_decimal(&word) {
                                if let Ok(num) = word.parse::<f64>() {
                                    Token::Float(num)
                                } else {
                                    Token::Word(word)
                                }
                            } else if Self::is_number(&word) {
                                if let Ok(num) = word.parse::<i64>() {
                                    Token::Num(num)
                                } else {
                                    Token::Word(word)
                                }
                            } else {
                                Token::Word(word)
                            }
                        }
                    };
                    
                    tokens.push(SpannedToken { token, span: Span { start, end } });
                }
            }
        }
        
        tokens.push(SpannedToken {
            token: Token::EOF,
            span: Span { start: self.input_len, end: self.input_len },
        });
        
        Ok(tokens)
    }
}
