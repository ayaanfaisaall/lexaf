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

/// A lexical analyzer that converts a raw string input into a sequence of zero-copy tokens.
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` instance from the given string input.
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(), 
        }
    }

    /// Retrieves the current byte index from the underlying character iterator, 
    /// or returns the total input length if the iterator is exhausted.
    fn pos(&mut self) -> usize {
        self.chars.peek().map(|&(idx, _)| idx).unwrap_or(self.input.len())
    }

    /// Consumes the input string and constructs a vector of spanned tokens.
    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken<'a>>, LexafError> {
        let mut tokens = Vec::new();
        
        while let Some(&(start, c)) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => { self.chars.next(); }
                '\n' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::NewLine, span: Span { start, end: start + 1 } });
                }
                '\'' | '"' => {
                    tokens.push(self.lex_string(start, c)?);
                }
                ';' => { self.chars.next(); tokens.push(SpannedToken { token: Token::SemiCln, span: Span { start, end: start + 1 } }); }
                ',' => { self.chars.next(); tokens.push(SpannedToken { token: Token::Comma, span: Span { start, end: start + 1 } }); }
                '{' => { self.chars.next(); tokens.push(SpannedToken { token: Token::LBrc, span: Span { start, end: start + 1 } }); }
                '}' => { self.chars.next(); tokens.push(SpannedToken { token: Token::RBrc, span: Span { start, end: start + 1 } }); }
                '[' => { self.chars.next(); tokens.push(SpannedToken { token: Token::LSqr, span: Span { start, end: start + 1 } }); }
                ']' => { self.chars.next(); tokens.push(SpannedToken { token: Token::RSqr, span: Span { start, end: start + 1 } }); }
                '<' => { self.chars.next(); tokens.push(SpannedToken { token: Token::RdrctIn, span: Span { start, end: start + 1 } }); }
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
                        tokens.push(SpannedToken { token: Token::AppendBoth, span: Span { start, end: start + 2 } });
                    } else {
                        tokens.push(SpannedToken { token: Token::RdrctBoth, span: Span { start, end: start + 1 } });
                    }
                }
                '#' => {
                    self.chars.next();
                    while let Some(&(_, ch)) = self.chars.peek() {
                        if ch == '\n' { break; }
                        self.chars.next();
                    }
                }
                '$' => {
                    self.lex_eval_block(start, &mut tokens)?;
                }
                'o' | 'e' => {
                    self.chars.next();
                    
                    if let Some(&(_, '>')) = self.chars.peek() {
                        self.chars.next();
                        
                        if let Some(&(_, '>')) = self.chars.peek() {
                            self.chars.next();
                            let token = if c == 'o' { Token::AppendOut } else { Token::AppendErr };
                            tokens.push(SpannedToken { token, span: Span { start, end: self.pos() } });
                        } else {
                            let token = if c == 'o' { Token::RdrctOut } else { Token::RdrctErr };
                            tokens.push(SpannedToken { token, span: Span { start, end: self.pos() } });
                        }
                    } else {
                        tokens.push(self.lex_word(start));
                    }
                }
                _ => {
                    tokens.push(self.lex_word(start));
                }
            }
        }
        
        tokens.push(SpannedToken {
            token: Token::EOF,
            span: Span { start: self.input.len(), end: self.input.len() },
        });
        
        Ok(tokens)
    }

    /// Handles lexing standard unquoted words/keywords outside of eval blocks.
    fn lex_word(&mut self, start: usize) -> SpannedToken<'a> {
        while let Some(&(_, ch)) = self.chars.peek() {
            match ch {
                  ' ' | '\n' | '\t' | '\r' | '"' | '='
                | ';' | ','  | '&'  | '|'  | '!' | '#'
                | '['  | ']'  | '$' | '<' 
                | '>' | '\'' => break,
                '{' => {
                    if let Some(&(_,'{')) = self.chars.peek() {
                        self.chars.next();
                        continue;
                    } else {
                        break;
                    }
                }
                '}' => {
                    if let Some(&(_,'}')) = self.chars.peek() {
                        self.chars.next();
                        continue;
                    } else {
                        break;
                    }
                }
                _ => { self.chars.next(); }
            }
        }
        
        let end = self.pos();
        let word = &self.input[start..end];
        let token = Self::lex_keyword(word);
        
        SpannedToken { token, span: Span { start, end } }
    }

    /// Parses a given string slice into a Float, Num, or falls back to a Word token.
    fn lex_num_float(word: &'a str) -> Token<'a> {
        if let Ok(num) = word.parse::<i64>() {
            Token::Num(num)
        } else if let Ok(num) = word.parse::<f64>() {
            Token::Float(num)
        } else {
            Token::Word(word)
        }
    }

    /// Checks if a word is a keyword, otherwise falls back to number/word logic.
    fn lex_keyword(word: &'a str) -> Token<'a> {
        match word {
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
            _ => Self::lex_num_float(word),
        }
    }

    /// Handles both single and double quoted strings, including `{}` interpolation logic for double quotes.
    fn lex_string(&mut self, start: usize, delimiter: char) -> Result<SpannedToken<'a>, LexafError> {
        self.chars.next();
        let mut str_tokens = Vec::new();
        let mut lit_start = self.pos();

        loop {
            match self.chars.next() {
                Some((idx, ch)) if ch == delimiter => {
                    if lit_start < idx {
                        str_tokens.push(StrIntr::Literal(&self.input[lit_start..idx]));
                    }
                    break;
                }
                Some((idx, '{')) if delimiter == '"' => {
                    if lit_start < idx {
                        str_tokens.push(StrIntr::Literal(&self.input[lit_start..idx]));
                    }
                    
                    let var_start = self.pos();
                    let var_end = loop {
                        match self.chars.next() {
                            Some((c_idx, '}')) => break c_idx,
                            Some((_, '"')) | None => {
                                return Err(LexafError::UnclosedDelimiter {
                                    delimiter: "}".to_string(),
                                    span: (start..self.pos()).into(),
                                });
                            }
                            Some(_) => continue,
                        }
                    };
                    
                    str_tokens.push(StrIntr::Variable(&self.input[var_start..var_end]));
                    lit_start = self.pos();
                }
                Some(_) => continue,
                None => {
                    return Err(LexafError::UnclosedDelimiter {
                        delimiter: delimiter.to_string(),
                        span: (start..self.pos()).into(),
                    });
                }
            }
        }
        
        let end = self.pos();
        Ok(SpannedToken {
            token: Token::Str(str_tokens),
            span: Span { start, end },
        })
    }

    /// Handles parsing the isolated math/variable environment inside `$`. 
    fn lex_eval_block(&mut self, start: usize, tokens: &mut Vec<SpannedToken<'a>>) -> Result<(), LexafError> {
        self.chars.next();
        tokens.push(SpannedToken { token: Token::Eval, span: Span { start, end: start + 1 } });
        
        while let Some(&(idx, ch)) = self.chars.peek() {
            match ch {
                ' ' | '\n' | '\t' | '\r' => { self.chars.next(); }
                '{' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::LBrc, span: Span { start: idx, end: idx + 1 } });
                    break;
                }
                _ => {
                    let err_start = idx;
                    while let Some(&(_, c)) = self.chars.peek() {
                        if c.is_whitespace() || c == '{' { break; }
                        self.chars.next();
                    }
                    let err_end = self.pos();
                    let bad_token = self.input[err_start..err_end].to_string();
                    
                    return Err(LexafError::UnexpectedToken {
                        token: bad_token,
                        span: (err_start..err_end).into(),
                    });
                }
            }
        }

        let mut closed = false;
        while let Some(&(inner_start, ch)) = self.chars.peek() {
            match ch { 
                ' ' | '\n' | '\t' | '\r' => { self.chars.next(); } 
                '}' => {
                    self.chars.next();
                    tokens.push(SpannedToken { token: Token::RBrc, span: Span { start: inner_start, end: inner_start + 1 } });
                    closed = true;
                    break;
                }
                '(' => { self.chars.next(); tokens.push(SpannedToken { token: Token::LPths, span: Span { start: inner_start, end: inner_start + 1 } }); }
                ')' => { self.chars.next(); tokens.push(SpannedToken { token: Token::RPths, span: Span { start: inner_start, end: inner_start + 1 } }); }
                '+' => { self.chars.next(); tokens.push(SpannedToken { token: Token::Plus, span: Span { start: inner_start, end: inner_start + 1 } }); }
                '-' => { self.chars.next(); tokens.push(SpannedToken { token: Token::Minus, span: Span { start: inner_start, end: inner_start + 1 } }); }
                '*' => { self.chars.next(); tokens.push(SpannedToken { token: Token::Multiply, span: Span { start: inner_start, end: inner_start + 1 } }); }
                '/' => { self.chars.next(); tokens.push(SpannedToken { token: Token::Divide, span: Span { start: inner_start, end: inner_start + 1 } }); }
                '%' => { self.chars.next(); tokens.push(SpannedToken { token: Token::Modulo, span: Span { start: inner_start, end: inner_start + 1 } }); }
                '^' => { self.chars.next(); tokens.push(SpannedToken { token: Token::Power, span: Span { start: inner_start, end: inner_start + 1 } }); }
                
                '=' | ';' | ',' | '&' | '|' | '!' | '<' | '>' | '#' | '"' | '\'' | '[' | ']' | '{' | '$' => {
                    self.chars.next();
                    return Err(LexafError::OperNotAllowed {
                        operator: ch.to_string(),
                        span: (inner_start..inner_start + 1).into(),
                    });
                }
                
                _ => {
                    while let Some(&(_, mw)) = self.chars.peek() {
                        match mw {
                              ' ' | '\n' | '\t' | '\r' 
                            | '}' | '(' | ')' | '+' | '-' | '*' | '/' | '%' | '^'
                            | '=' | ';' | ',' | '&' | '|' | '!' | '<' | '>' | '#' 
                            | '"' | '\'' | '[' | ']' | '{' | '$' => {
                                break;
                            }
                            _ => {
                                self.chars.next();
                            }
                        }
                    }
                    
                    let inner_end = self.pos();
                    let word = &self.input[inner_start..inner_end];
                    
                    let token = Self::lex_num_float(word);
                    tokens.push(SpannedToken { token, span: Span { start: inner_start, end: inner_end } });
                }
            }
        } 
        
        if !closed {
            return Err(LexafError::UnclosedDelimiter {
                delimiter: "}".to_string(),
                span: (start..self.pos()).into(), 
            });
        }
        
        Ok(())
    }

}
