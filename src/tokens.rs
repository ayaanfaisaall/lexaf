//
// note: () or parenthesis are intentionally not 
// allowed in my shell and its language and hence
// (), don't even have a Token ( except ${ } ) :)
//
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // general
    Word(String),
    Str(Vec<StrIntr>),
    Num(i64),
    // symbols
    NewLine,
    Eval,
    // punctuation
    SemiCln,
    Comma,
    And,
    // logical operators
    AndAnd,
    OrOr,
    Bang,
    //comparison & assign operators
    Assign,
    EqEq,
    NotEq,
    // shell operators
    Pipe,
    RdrctIn,
    RdrctOut,
    Append,
    // brackets
    LBrc,
    RBrc,
    LSqr,
    RSqr,
    // lang keywords
    Let,
    Print,
    If,
    Elif,
    Else,
    For,
    While,
    In,
    To,
    Break,
    True,
    False,
    // equality operators
    // EqualTo, 
    // LessEqual, 
    // LessThan,
    // GreaterEqual,
    // GreaterThan,
    //math operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    LPths,
    RPths,
    // end of file
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrIntr {
    Literal(String),
    Variable(String),
}
//
// EOF :)
//
