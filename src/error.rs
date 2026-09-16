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
}
