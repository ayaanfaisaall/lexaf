use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum LexafError {
    #[error("unclosed delimiter: '{delimiter}'")]
    #[diagnostic(
        code(afsh::lexaf::unclosed_delimiter),
        help("close: '{delimiter}' properly")
    )]
    UnclosedDelimiter {
        delimiter: String,
        #[label("'{delimiter}' might not be closed properly")]
        span: SourceSpan,
    },

    #[error("operator not allowed in eval block: '{operator}'")]
    #[diagnostic(
        code(afsh::lexaf::oper_not_allowed),
        help("only math operators like +, -, *, /, %, and ^ are allowed inside $ {{ ... }}")
    )]
    OperNotAllowed {
        operator: String,
        #[label("invalid operator '{operator}' used here")]
        span: SourceSpan,
    },

    #[error("unexpected token: '{token}'")]
    #[diagnostic(
        code(afsh::lexaf::unexpected_token),
        help("expected a valid syntax token (e.g., ensure '$' is followed by '{{')")
    )]
    UnexpectedToken {
        token: String,
        #[label("unexpected '{token}' found here")]
        span: SourceSpan,
    },
}
